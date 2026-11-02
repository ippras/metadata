pub mod r#const;
pub mod join;
pub mod l10n;
pub mod rule;

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "polars")]
pub mod polars;

use crate::rule::{ALL_IDENTIFIERS, ALL_NAMES, ALL_VERSIONS, LAST_DATE, Order, Rule, Value};
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    collections::HashSet,
    fmt::{Debug, Display, Formatter, from_fn},
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

pub const ID_SALT: &str = "Metadata";

pub static PRETTY_CONFIG: LazyLock<PrettyConfig> = LazyLock::new(|| {
    PrettyConfig::new()
        .depth_limit(2)
        // .extensions(Extensions::UNWRAP_NEWTYPES | Extensions::IMPLICIT_SOME)
        .extensions(Extensions::UNWRAP_NEWTYPES)
        .new_line("\n")
});

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(Vec<Parameter>);

impl Metadata {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_name_value(&mut self, name: impl Into<String>, value: impl ToString) {
        self.0.push(Parameter {
            name: name.into(),
            value: value.to_string(),
        });
    }

    pub fn filter_name(&self, name: &str) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .filter_map(move |parameter| (parameter.name == name).then_some(&*parameter.value))
    }

    pub fn find_name(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .find_map(|parameter| (parameter.name == name).then_some(&*parameter.value))
    }

    pub fn contains_name_value(&self, name: &str, value: &str) -> bool {
        self.0
            .iter()
            .find(|parameter| parameter.name == name && parameter.value == value)
            .is_some()
    }

    /// Оставляет только те параметры, которые попали хотя бы под одно правило,
    /// фильтрует дубликаты согласно `Rule::value` и выстраивает их в порядке правил.
    pub fn filter_and_sort<T: Borrow<Rule>>(&mut self, rules: impl AsRef<[T]>) {
        let rules = rules.as_ref();
        let parameters = std::mem::take(&mut self.0);

        // Фильтруем по имени
        // Находим первое подходящее правило для каждого параметра
        let rank_value_sort_parameters: Vec<_> = parameters
            .into_iter()
            .filter_map(|parameter| {
                // Ищем подходящее правило и захватываем его политику сортировки
                let (rank, value, sort) = rules.iter().enumerate().find_map(|(rank, rule)| {
                    let rule = rule.borrow();
                    if rule.name.matches(&parameter.name) {
                        Some((rank, rule.value, rule.sort))
                    } else {
                        None
                    }
                })?;
                Some((rank, value, sort, parameter))
            })
            .collect();

        // Применяем политики First, Last и Unique
        let mut keep = vec![true; rank_value_sort_parameters.len()];
        let mut first = HashSet::new();
        let mut last = HashSet::new();
        let mut unique = HashSet::new();

        // Обрабатываем First и Unique (идем с начала)
        for (index, (_, value, _, parameter)) in rank_value_sort_parameters.iter().enumerate() {
            match value {
                Value::First => {
                    if !first.insert(&parameter.name) {
                        keep[index] = false;
                    }
                }
                Value::Unique => {
                    if !unique.insert((&parameter.name, &parameter.value)) {
                        keep[index] = false;
                    }
                }
                _ => {}
            }
        }

        // Обрабатываем Last (идем с конца)
        for (index, (_, value, _, parameter)) in rank_value_sort_parameters.iter().enumerate().rev()
        {
            if let Value::Last = value {
                if !last.insert(&parameter.name) {
                    keep[index] = false;
                }
            }
        }

        // Фильтруем по значению
        let mut rank_parameters: Vec<_> = rank_value_sort_parameters
            .into_iter()
            .enumerate()
            .filter_map(|(index, (rank, _, sort, parameter))| {
                if keep[index] {
                    Some((rank, sort, parameter))
                } else {
                    None
                }
            })
            .collect();

        // Сортируем
        // rank_parameters.sort_by_cached_key(|(rank, sort, parameter)| {
        //     (*rank, *sort, *parameter)
        //     // left_rank
        //     //     .cmp(right_rank)
        //     //     // Если параметры попали под одно и то же правило (ранги равны)
        //     //     .then_with(|| {
        //     //         // Поскольку left_rank == right_rank, то и left_sort == right_sort
        //     //         match left_sort {
        //     //             Some(Order::Ascending) => left_parameter
        //     //                 .name
        //     //                 .cmp(&right_parameter.name)
        //     //                 .then_with(|| left_parameter.value.cmp(&right_parameter.value)),
        //     //             // Возвращаем Equal, чтобы стабильная сортировка сохранила исходный порядок
        //     //             None => std::cmp::Ordering::Equal,
        //     //         }
        //     //     })
        // });
        rank_parameters.sort_by(
            |(left_rank, left_sort, left_parameter), (right_rank, _right_sort, right_parameter)| {
                left_rank
                    .cmp(right_rank)
                    // Если параметры попали под одно и то же правило (ранги равны)
                    .then_with(|| {
                        // Поскольку left_rank == right_rank, то и left_sort == right_sort
                        match left_sort {
                            Some(Order::Ascending) => left_parameter
                                .name
                                .cmp(&right_parameter.name)
                                .then_with(|| left_parameter.value.cmp(&right_parameter.value)),
                            Some(Order::Descending) => right_parameter
                                .name
                                .cmp(&left_parameter.name)
                                .then_with(|| right_parameter.value.cmp(&left_parameter.value)),
                            // Возвращаем Equal, чтобы стабильная сортировка сохранила исходный порядок
                            None => std::cmp::Ordering::Equal,
                        }
                    })
            },
        );

        // Убираем ранг и возвращаем обратно в self
        self.0 = rank_parameters
            .into_iter()
            .map(|(_, _, parameter)| parameter)
            .collect();
    }

    pub fn format(&self) -> impl Display {
        let mut parameters = self.clone();
        parameters.filter_and_sort([&*ALL_NAMES, &*ALL_IDENTIFIERS, &*LAST_DATE, &*ALL_VERSIONS]);
        from_fn(move |f| write!(f, "{parameters}"))
    }

    pub fn format_with<T: Borrow<Rule>>(&self, rules: impl AsRef<[T]>) -> impl Display {
        let mut parameters = self.clone();
        parameters.filter_and_sort(rules);
        from_fn(move |f| write!(f, "{parameters}"))
    }
}

impl Deref for Metadata {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for parameter in &self.0 {
            write!(f, "{{{parameter}}}")?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Metadata {
    type Item = &'a Parameter;
    type IntoIter = std::slice::Iter<'a, Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for Metadata {
    type Item = Parameter;
    type IntoIter = std::vec::IntoIter<Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Parameter
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Parameter<T = String, U = String> {
    pub name: T,
    pub value: U,
}

impl Parameter {
    fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)?;
        Ok(())
    }
}

/// Version
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Version(pub u64, pub u64, pub u64);

impl Version {
    fn new() -> Self {
        Self(0, 0, 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.0 != 0 {
            write!(f, "{}", self.0)?;
        }
        if self.1 != 0 {
            if self.0 != 0 {
                f.write_str(".")?;
            }
            write!(f, "{}", self.1)?;
        }
        if self.0 != 0 || self.1 != 0 {
            f.write_str(".")?;
        }
        write!(f, "{}", self.2)?;
        Ok(())
    }
}
