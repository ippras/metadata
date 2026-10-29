use crate::r#const::{
    AUTHOR, DATE, DESCRIPTION, IDENTIFIER, NAME, STEREOSPECIFIC_NUMBERS, VERSION,
};
use std::sync::LazyLock;
use typed_builder::TypedBuilder;

pub static UNIQUE_AUTHORS: LazyLock<Rule> = LazyLock::new(|| {
    Rule::builder()
        .name(Name::exact(AUTHOR))
        .value(Value::Unique)
        .build()
});

pub static UNIQUE_AUTHORS_AND_UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_UNIQUE_KINDS_AND_DATES_AND_VERSIONS: LazyLock<
    [Rule; 6],
> = LazyLock::new(|| {
    [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(STEREOSPECIFIC_NUMBERS))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .value(Value::All)
            .build(),
    ]
});

pub static UNIQUE_AUTHORS_AND_ALL_NAMES_AND_UNIQUE_IDENTIFIERS_AND_UNIQUE_KINDS_AND_DATES_AND_VERSIONS: LazyLock<
    [Rule; 6],
> = LazyLock::new(|| {
    [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(STEREOSPECIFIC_NUMBERS))
            .value(Value::Unique)
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .value(Value::All)
            .build(),
    ]
});

pub static NAMES_AND_IDENTIFIERS_AND_LAST_DATE_AND_VERSIONS: LazyLock<[Rule; 4]> =
    LazyLock::new(|| {
        [
            Rule::builder()
                .name(Name::exact(NAME))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(IDENTIFIER))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(DATE))
                .value(Value::Last)
                .build(),
            Rule::builder()
                .name(Name::exact(VERSION))
                .value(Value::All)
                .build(),
        ]
    });

pub static KINDS_AND_NAMES_AND_IDENTIFIERS_AND_LAST_DATE_AND_VERSIONS: LazyLock<[Rule; 5]> =
    LazyLock::new(|| {
        [
            Rule::builder()
                .name(Name::exact(STEREOSPECIFIC_NUMBERS))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(NAME))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(IDENTIFIER))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(DATE))
                .value(Value::Last)
                .build(),
            Rule::builder()
                .name(Name::exact(VERSION))
                .value(Value::All)
                .build(),
        ]
    });

pub static SPECIAL_ALPHABETICAL_RULES: LazyLock<[Rule; 8]> = LazyLock::new(|| {
    [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(DESCRIPTION))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(STEREOSPECIFIC_NUMBERS))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::All)
            .value(Value::All)
            .sort(Order::Ascending)
            .build(),
    ]
});

/// Order
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

/// Rule
#[derive(Clone, Eq, PartialEq, TypedBuilder)]
pub struct Rule {
    pub name: Name,
    pub value: Value,
    #[builder(default, setter(strip_option))]
    pub sort: Option<Order>,
}

/// Name
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Name {
    /// Точное совпадение имени параметра
    Exact(String),
    /// Логическое NOT - всё, кроме указанных в списке
    Not(Box<Self>),
    /// Логическое ИЛИ для сложных вложенных правил
    Or(Vec<Name>),
    /// Логическое И для сложных вложенных правил
    And(Vec<Name>),
    /// Захватить вообще всё
    All,
}

impl Name {
    pub fn exact(name: impl Into<String>) -> Self {
        Name::Exact(name.into())
    }

    pub fn not(expr: Name) -> Self {
        Name::Not(Box::new(expr))
    }

    pub fn or(exprs: impl IntoIterator<Item = Name>) -> Self {
        Name::Or(exprs.into_iter().collect())
    }

    pub fn and(exprs: impl IntoIterator<Item = Name>) -> Self {
        Name::And(exprs.into_iter().collect())
    }
}

impl Name {
    /// Проверяет, подходит ли имя параметра под данное выражение
    pub fn matches(&self, target: &str) -> bool {
        match self {
            Name::Exact(name) => target == name,
            Name::Not(name) => !name.matches(target),
            Name::Or(names) => names.iter().any(|name| name.matches(target)),
            Name::And(names) => names.iter().all(|name| name.matches(target)),
            Name::All => true,
        }
    }
}

/// Value
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    #[default]
    All,
    First,
    Last,
    Unique,
}
