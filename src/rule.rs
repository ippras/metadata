/// Rule
pub struct Rule {
    pub name: Name,
    pub value: Value,
}

impl Rule {
    pub fn new(name: Name, value: Value) -> Self {
        Self { name, value }
    }
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
    pub fn matches(&self, name: &str) -> bool {
        match self {
            Name::Exact(n) => name == n,
            Name::Not(expr) => !expr.matches(name),
            Name::Or(exprs) => exprs.iter().any(|expr| expr.matches(name)),
            Name::And(exprs) => exprs.iter().all(|expr| expr.matches(name)),
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
}
