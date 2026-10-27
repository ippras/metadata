use crate::r#const::{AUTHOR, DATE, DESCRIPTION, IDENTIFIER, NAME, VERSION};
use std::sync::LazyLock;
use typed_builder::TypedBuilder;

pub static UNIQUE_NAMES_AND_UNIQUE_IDENTIFIERS_AND_ALL_DATES_AND_ALL_VERSIONS: LazyLock<[Rule; 4]> =
    LazyLock::new(|| {
        [
            Rule::builder()
                .name(Name::exact(NAME))
                .value(Value::Unique)
                .build(),
            Rule::builder()
                .name(Name::exact(IDENTIFIER))
                .value(Value::Unique)
                .build(),
            Rule::builder()
                .name(Name::exact(DATE))
                .value(Value::All)
                .build(),
            Rule::builder()
                .name(Name::exact(VERSION))
                .value(Value::All)
                .build(),
        ]
    });

pub static NAME_AND_IDENTIFIER_AND_DATE_LAST_AND_VERSION: LazyLock<[Rule; 4]> =
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

pub static SPECIAL_ALPHABETICAL_RULES: LazyLock<[Rule; 7]> = LazyLock::new(|| {
    [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(DESCRIPTION))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .value(Value::All)
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .value(Value::All)
            .build(),
        Rule::builder().name(Name::All).value(Value::All).build(),
    ]
});

/// Order
#[derive(Clone, Copy, Debug, Default)]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

/// Rule
#[derive(TypedBuilder)]
pub struct Rule {
    pub name: Name,
    pub value: Value,
    #[builder(default, setter(strip_option))]
    pub sort: Option<Order>,
}

/// Name
#[derive(Clone, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub enum Value {
    All,
    First,
    Last,
    Unique,
}
