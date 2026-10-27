pub mod r#const;
pub mod join;
pub mod l10n;
pub mod rule;

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "polars")]
pub mod polars;

use crate::rule::{NAMES_AND_IDENTIFIERS_AND_LAST_DATE_AND_VERSIONS, Name, Order, Rule, Value};
use jiff::civil::Date;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Display, Formatter, from_fn},
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    sync::LazyLock,
};

pub const ID_SALT: &str = "Metadata";

pub static PRETTY_CONFIG: LazyLock<PrettyConfig> = LazyLock::new(|| {
    PrettyConfig::new()
        .depth_limit(2)
        .extensions(Extensions::UNWRAP_NEWTYPES | Extensions::IMPLICIT_SOME)
        .new_line("\n")
});

// /// Metadata
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct Metadata {
//     #[serde(default, skip_serializing_if = "String::is_empty")]
//     pub name: String,
//     #[serde(default, skip_serializing_if = "String::is_empty")]
//     pub description: String,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub authors: Vec<String>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub dates: Vec<Date>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub versions: Vec<Version>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     pub parameters: Vec<Parameter>,
// }

// impl TryFrom<Metadata> for BTreeMap<String, String> {
//     type Error = ron::Error;

//     fn try_from(value: Metadata) -> Result<Self, Self::Error> {
//         let mut map = BTreeMap::new();
//         if !value.name.is_empty() {
//             map.insert(NAME.to_owned(), value.name);
//         }
//         if !value.description.is_empty() {
//             map.insert(DESCRIPTION.to_owned(), value.description);
//         }
//         if !value.authors.is_empty() {
//             map.insert(AUTHORS.to_owned(), ron::ser::to_string(&value.authors)?);
//         }
//         if !value.parameters.is_empty() {
//             map.insert(
//                 PARAMETERS.to_owned(),
//                 ron::ser::to_string(&value.parameters)?,
//             );
//         }
//         if !value.versions.is_empty() {
//             map.insert(VERSIONS.to_owned(), ron::ser::to_string(&value.versions)?);
//         }
//         if !value.dates.is_empty() {
//             map.insert(DATES.to_owned(), ron::ser::to_string(&value.dates)?);
//         }
//         Ok(map)
//     }
// }

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(Vec<Parameter>);

impl Metadata {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_name_value(&mut self, name: impl Into<String>, value: Option<impl ToString>) {
        self.0.push(Parameter {
            name: name.into(),
            value: value.map(|value| value.to_string()),
        });
    }

    /// Оставляет только те параметры, которые попали хотя бы под одно правило,
    /// фильтрует дубликаты согласно `Rule::value` и выстраивает их в порядке правил.
    pub fn filter_and_sort(&mut self, rules: &[Rule]) {
        let parameters = std::mem::take(&mut self.0);

        // Фильтруем по имени
        // Находим первое подходящее правило для каждого параметра
        let rank_value_sort_parameters: Vec<_> = parameters
            .into_iter()
            .filter_map(|parameter| {
                // Ищем подходящее правило и захватываем его политику сортировки
                let (rank, value, sort) = rules.iter().enumerate().find_map(|(rank, rule)| {
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
        parameters.filter_and_sort(&*NAMES_AND_IDENTIFIERS_AND_LAST_DATE_AND_VERSIONS);
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
pub struct Parameter {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Parameter {
    fn new() -> Self {
        Self {
            name: String::new(),
            value: None,
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value) = &self.value {
            write!(f, "={value}")?;
        }
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

const GVK: &str = "Giorgi Vladimirovich Kazakov";
const RAS: &str = "Roman Alexandrovich Sidorov";

#[cfg(test)]
mod test {
    use super::*;
    use crate::{r#const::*, rule::Value};
    use jiff::civil::date;

    #[test]
    fn test() {
        let mut parameters = Metadata::new();
        parameters.push_name_value(NAME, Some("VIR-2699"));
        parameters.push_name_value(DATE, Some(date(2026, 09, 11)));
        parameters.push_name_value(DATE, Some(date(2026, 09, 09)));
        parameters.push_name_value(DESCRIPTION, Some("Cat. No. 2699, Прогресс, Россия"));
        parameters.push_name_value(AUTHOR, Some(RAS));
        parameters.push_name_value(AUTHOR, Some(GVK));
        parameters.push_name_value(VERSION, Some(Version(1, 2, 3)));
        parameters.push_name_value(VERSION, Some(Version(0, 1, 2)));
        parameters.push_name_value("CustomName", Some("CustomValue"));
        println!("parameters: {parameters:#?}");

        parameters.filter_and_sort(&[
            // 1. Сначала `DATE`
            Rule::builder()
                .name(Name::exact(DATE))
                .value(Value::Last)
                .build(),
            // 2. Затем `NAME`
            Rule::builder()
                .name(Name::or([Name::exact(NAME)]))
                .value(Value::All)
                .build(),
            // 3. Затем всё остальное, КРОМЕ `AUTHOR`, `DESCRIPTION`, `VERSION`
            Rule::builder()
                .name(Name::and([
                    Name::All,
                    Name::not(Name::or(vec![
                        Name::exact(AUTHOR),
                        Name::exact(DESCRIPTION),
                        Name::exact(VERSION),
                    ])),
                ]))
                .value(Value::All)
                .build(),
            // 4. В самом конце `VERSION`
            Rule::builder()
                .name(Name::exact(VERSION))
                .value(Value::All)
                .build(),
        ]);
        println!("debug: {parameters:#?}");
        println!("display: {parameters}");

        let contents = ron::ser::to_string_pretty(&parameters, PRETTY_CONFIG.clone()).unwrap();
        std::fs::write("path.ron", &contents).unwrap();
        println!("contents: {contents}");
    }

    // #[test]
    // fn test() {
    //     let meta = Metadata {
    //         name: "VIR-2699".to_owned(),
    //         description: "Cat. No. 2699, Прогресс, Россия".to_owned(),
    //         authors: vec![GVK.to_owned(), RAS.to_owned()],
    //         parameters: Vec::new(),
    //         versions: vec![Version(1, 2, 3), Version(0, 1, 2)],
    //         dates: vec![date(2026, 09, 09), date(2026, 09, 11)],
    //     };
    //     println!("meta: {meta:#?}");

    //     // println!(r#"Dates::All: "{}""#, meta.format().date(Value::All).build());
    //     // println!(
    //     //     r#"Dates::First: "{}""#,
    //     //     meta.format().date(Value::First).build()
    //     // );
    //     // println!(
    //     //     r#"Dates::Last: "{}""#,
    //     //     meta.format().date(Value::Last).build()
    //     // );
    //     // println!(r#"default: "{}""#, meta.format().build());
    //     println!(r#"default: "{}""#, meta.format().build());
    //     println!(
    //         r#"All: "{}""#,
    //         meta.format()
    //             .parameter_format(Some(ParameterFormat::new(DATE, Value::All)))
    //             .parameter_format(Some(ParameterFormat::new(NAME, Value::All)))
    //             .parameter_format(None)
    //             .parameter_format(Some(ParameterFormat::new(VERSION, Value::All)))
    //     );

    //     let contents = ron::ser::to_string_pretty(&meta, PRETTY_CONFIG.clone()).unwrap();
    //     std::fs::write("path.ron", &contents).unwrap();
    //     println!("contents: {contents:?}");
    // }

    // #[test]
    // fn default() {
    //     let meta = Metadata::default();
    //     println!(
    //         r#"Dates::All: "{}""#,
    //         meta.format().date(Value::All).build()
    //     );
    //     println!(
    //         r#"Dates::First: "{}""#,
    //         meta.format().date(Value::First).build()
    //     );
    //     println!(
    //         r#"Dates::Last: "{}""#,
    //         meta.format().date(Value::Last).build()
    //     );
    //     println!(r#"meta: "{}""#, meta.format().build());
    // }
}
