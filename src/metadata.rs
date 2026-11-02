use crate::{
    Parameter,
    rule::{ALL_IDENTIFIERS, ALL_NAMES, ALL_VERSIONS, LAST_DATE, Order, Rule, Value},
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Упорядоченный набор параметров метаданных.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(Vec<Parameter>);

impl Metadata {
    /// Пустой набор метаданных.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Добавляет параметр, превращая значение в строку через [`ToString`].
    pub fn push_name_value(&mut self, name: impl Into<String>, value: impl ToString) {
        self.0.push(Parameter::new(name, value.to_string()));
    }

    /// Значения всех параметров с данным именем в порядке следования.
    pub fn filter_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a str> {
        self.0
            .iter()
            .filter(move |parameter| parameter.name == name)
            .map(|parameter| parameter.value.as_str())
    }

    /// Значение первого параметра с данным именем.
    pub fn find_name<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        self.filter_name(name).next()
    }

    /// Есть ли параметр с точно такой парой «имя — значение».
    pub fn contains_name_value(&self, name: &str, value: &str) -> bool {
        self.0
            .iter()
            .any(|parameter| parameter.name == name && parameter.value == value)
    }

    /// Сравнивает две метаданные по списку полей: сначала по первому полю,
    /// при равенстве по следующему и т. д. Если поля нет, значений ноль,
    /// и такая запись идёт раньше (при `Descending` наоборот, позже).
    pub fn cmp_by<S: AsRef<str>>(&self, other: &Self, sort: &[(S, Order)]) -> Ordering {
        sort.iter()
            .map(|(name, order)| {
                let name = name.as_ref();
                order.apply(self.filter_name(name).cmp(other.filter_name(name)))
            })
            .find(|ordering| ordering.is_ne())
            .unwrap_or(Ordering::Equal)
    }

    /// Оставляет параметры, попавшие хотя бы под одно правило, убирает дубликаты
    /// согласно [`Rule::value`] и выстраивает результат в порядке правил.
    ///
    /// Для каждого параметра берётся первое подходящее правило. Параметры, под
    /// которые не подошло ни одно правило, удаляются.
    pub fn filter_and_sort<T: Borrow<Rule>>(&mut self, rules: impl AsRef<[T]>) {
        let rules = rules.as_ref();
        let ranked: Vec<Ranked> = std::mem::take(&mut self.0)
            .into_iter()
            .filter_map(|parameter| Ranked::bind(parameter, rules))
            .collect();

        let flags = retention_flags(&ranked);
        let mut kept: Vec<Ranked> = ranked
            .into_iter()
            .zip(flags)
            .filter_map(|(ranked, keep)| keep.then_some(ranked))
            .collect();

        kept.sort_by(Ranked::cmp_by_rule);
        self.0 = kept.into_iter().map(|ranked| ranked.parameter).collect();
    }

    /// Форматирует метаданные стандартным набором правил.
    pub fn format(&self) -> impl Display {
        self.format_with([&*ALL_NAMES, &*ALL_IDENTIFIERS, &*LAST_DATE, &*ALL_VERSIONS])
    }

    /// Форматирует метаданные по заданным правилам.
    pub fn format_with<T: Borrow<Rule>>(&self, rules: impl AsRef<[T]>) -> impl Display {
        let mut metadata = self.clone();
        metadata.filter_and_sort(rules);
        metadata
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|parameter| write!(f, "{{{parameter}}}"))
    }
}

impl FromIterator<Parameter> for Metadata {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
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

/// Параметр вместе с применённым к нему правилом.
struct Ranked {
    rank: usize,
    value: Value,
    sort: Option<Order>,
    parameter: Parameter,
}

impl Ranked {
    /// Привязывает параметр к первому подходящему правилу; без правила `None`.
    fn bind<T: Borrow<Rule>>(parameter: Parameter, rules: &[T]) -> Option<Self> {
        let (rank, rule) = rules
            .iter()
            .enumerate()
            .map(|(rank, rule)| (rank, Borrow::<Rule>::borrow(rule)))
            .find(|(_, rule)| rule.name.matches(&parameter.name))?;
        Some(Self {
            rank,
            value: rule.value,
            sort: rule.sort,
            parameter,
        })
    }

    /// Сначала по рангу правила; внутри одного правила (у них одна политика
    /// сортировки) по `(name, value)`, если сортировка задана. `sort_by` стабилен.
    fn cmp_by_rule(left: &Self, right: &Self) -> Ordering {
        left.rank.cmp(&right.rank).then_with(|| match left.sort {
            Some(order) => order.apply(left.parameter.cmp(&right.parameter)),
            None => Ordering::Equal,
        })
    }
}

/// Какие элементы оставить по политикам `First`, `Last` и `Unique`.
fn retention_flags(ranked: &[Ranked]) -> Vec<bool> {
    let mut flags = vec![true; ranked.len()];

    // `First` и `Unique`: идём с начала.
    let mut first_names = HashSet::new();
    let mut unique_pairs = HashSet::new();
    for (flag, item) in flags.iter_mut().zip(ranked) {
        let Parameter { name, value } = &item.parameter;
        *flag = match item.value {
            Value::First => first_names.insert(name.as_str()),
            Value::Unique => unique_pairs.insert((name.as_str(), value.as_str())),
            Value::All | Value::Last => true,
        };
    }

    // `Last`: идём с конца.
    let mut last_names = HashSet::new();
    for (flag, item) in flags.iter_mut().zip(ranked).rev() {
        if item.value == Value::Last && !last_names.insert(item.parameter.name.as_str()) {
            *flag = false;
        }
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        r#const::{AUTHOR, DATE, NAME},
        rule::Name,
    };

    fn metadata(pairs: &[(&str, &str)]) -> Metadata {
        pairs
            .iter()
            .map(|&(name, value)| Parameter::new(name, value))
            .collect()
    }

    fn pairs(metadata: &Metadata) -> Vec<(&str, &str)> {
        metadata
            .iter()
            .map(|parameter| (parameter.name.as_str(), parameter.value.as_str()))
            .collect()
    }

    #[test]
    fn drops_unmatched_and_orders_by_rule() {
        let mut metadata = metadata(&[("Date", "2"), ("Foo", "x"), ("Name", "n")]);
        metadata.filter_and_sort([Rule::exact(NAME, Value::All), Rule::exact(DATE, Value::All)]);
        assert_eq!(pairs(&metadata), [("Name", "n"), ("Date", "2")]);
    }

    #[test]
    fn first_keeps_first_per_name() {
        let mut metadata = metadata(&[("Date", "1"), ("Date", "2")]);
        metadata.filter_and_sort([Rule::exact(DATE, Value::First)]);
        assert_eq!(pairs(&metadata), [("Date", "1")]);
    }

    #[test]
    fn last_keeps_last_per_name() {
        let mut metadata = metadata(&[("Date", "1"), ("Date", "2")]);
        metadata.filter_and_sort([Rule::exact(DATE, Value::Last)]);
        assert_eq!(pairs(&metadata), [("Date", "2")]);
    }

    #[test]
    fn unique_drops_repeated_pairs() {
        let mut metadata = metadata(&[("Author", "a"), ("Author", "b"), ("Author", "a")]);
        metadata.filter_and_sort([Rule::exact(AUTHOR, Value::Unique)]);
        assert_eq!(pairs(&metadata), [("Author", "a"), ("Author", "b")]);
    }

    #[test]
    fn descending_sort_within_rule() {
        let mut metadata = metadata(&[("a", "1"), ("b", "1"), ("a", "2")]);
        let rule = Rule::builder()
            .name(Name::All)
            .sort(Order::Descending)
            .build();
        metadata.filter_and_sort([rule]);
        assert_eq!(pairs(&metadata), [("b", "1"), ("a", "2"), ("a", "1")]);
    }
}
