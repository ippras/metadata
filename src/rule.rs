//! Правила отбора, дедупликации и сортировки параметров метаданных.

use crate::r#const::{AUTHOR, DATE, IDENTIFIER, INDEX, NAME, VERSION};
use std::{cmp::Ordering, sync::LazyLock};
use typed_builder::TypedBuilder;

/// All
pub static ALL_ALL_ASCENDING: LazyLock<Rule> = LazyLock::new(|| {
    Rule::builder()
        .name(Name::All)
        .value(Value::All)
        .sort(Order::Ascending)
        .build()
});

/// All authors
pub static ALL_AUTHORS: LazyLock<Rule> = LazyLock::new(|| Rule::exact(AUTHOR, Value::All));
/// All dates
pub static ALL_DATES: LazyLock<Rule> = LazyLock::new(|| Rule::exact(DATE, Value::All));
/// All identifiers
pub static ALL_IDENTIFIERS: LazyLock<Rule> = LazyLock::new(|| Rule::exact(IDENTIFIER, Value::All));
/// All indexes
pub static ALL_INDEXES: LazyLock<Rule> = LazyLock::new(|| Rule::exact(INDEX, Value::All));
/// All names
pub static ALL_NAMES: LazyLock<Rule> = LazyLock::new(|| Rule::exact(NAME, Value::All));
/// All versions
pub static ALL_VERSIONS: LazyLock<Rule> = LazyLock::new(|| Rule::exact(VERSION, Value::All));
/// Last date
pub static LAST_DATE: LazyLock<Rule> = LazyLock::new(|| Rule::exact(DATE, Value::Last));
/// Unique authors
pub static UNIQUE_AUTHORS: LazyLock<Rule> = LazyLock::new(|| Rule::exact(AUTHOR, Value::Unique));
/// Unique dates
pub static UNIQUE_DATES: LazyLock<Rule> = LazyLock::new(|| Rule::exact(DATE, Value::Unique));
/// Unique identifiers
pub static UNIQUE_IDENTIFIERS: LazyLock<Rule> =
    LazyLock::new(|| Rule::exact(IDENTIFIER, Value::Unique));
/// Unique names
pub static UNIQUE_NAMES: LazyLock<Rule> = LazyLock::new(|| Rule::exact(NAME, Value::Unique));

/// Направление сортировки.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

impl Order {
    /// Применяет направление к результату сравнения «по возрастанию».
    pub const fn apply(self, ordering: Ordering) -> Ordering {
        match self {
            Self::Ascending => ordering,
            Self::Descending => ordering.reverse(),
        }
    }
}

/// Правило: какие параметры берём, как убираем дубликаты и как сортируем.
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Rule {
    /// Выражение над именем параметра.
    pub name: Name,
    /// Политика выбора среди параметров с одним именем.
    #[builder(default)]
    pub value: Value,
    /// Сортировка внутри правила; `None` сохраняет исходный порядок.
    #[builder(default, setter(strip_option))]
    pub sort: Option<Order>,
}

impl Rule {
    /// Правило для параметров с точным именем, без сортировки.
    pub fn exact(name: impl Into<String>, value: Value) -> Self {
        Self {
            name: Name::exact(name),
            value,
            sort: None,
        }
    }
}

/// Выражение над именем параметра.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Name {
    /// Точное совпадение имени.
    Exact(String),
    /// Отрицание вложенного выражения.
    Not(Box<Self>),
    /// Совпадает хотя бы одно вложенное выражение (пустой список не совпадает ни с чем).
    Or(Vec<Name>),
    /// Совпадают все вложенные выражения (пустой список совпадает со всем).
    And(Vec<Name>),
    /// Любое имя.
    All,
}

impl Name {
    pub fn exact(name: impl Into<String>) -> Self {
        Self::Exact(name.into())
    }

    pub fn not(expr: Self) -> Self {
        Self::Not(Box::new(expr))
    }

    pub fn or(exprs: impl IntoIterator<Item = Self>) -> Self {
        Self::Or(exprs.into_iter().collect())
    }

    pub fn and(exprs: impl IntoIterator<Item = Self>) -> Self {
        Self::And(exprs.into_iter().collect())
    }

    /// Подходит ли имя параметра под выражение.
    pub fn matches(&self, target: &str) -> bool {
        match self {
            Self::Exact(name) => target == name,
            Self::Not(name) => !name.matches(target),
            Self::Or(names) => names.iter().any(|name| name.matches(target)),
            Self::And(names) => names.iter().all(|name| name.matches(target)),
            Self::All => true,
        }
    }
}

/// Политика выбора среди параметров, подошедших под правило.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    /// Оставить все.
    #[default]
    All,
    /// Оставить только первый параметр с каждым именем.
    First,
    /// Оставить только последний параметр с каждым именем.
    Last,
    /// Убрать повторы одной и той же пары «имя — значение».
    Unique,
}
