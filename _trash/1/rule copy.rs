/// Rule
#[derive(Clone, Debug)]
pub enum Rule {
    /// Точное совпадение имени параметра
    Name(String),
    /// Отрицание (NOT): всё, кроме указанных в списке
    Not(Box<Self>),
    /// Логическое ИЛИ для сложных вложенных правил
    Or(Vec<Rule>),
    /// Логическое И для сложных вложенных правил
    And(Vec<Rule>),
    /// Захватить вообще всё
    All,
}

impl Rule {
    pub fn name(name: impl Into<String>) -> Self {
        Rule::Name(name.into())
    }

    pub fn not(expr: Rule) -> Self {
        Rule::Not(Box::new(expr))
    }

    pub fn or(exprs: impl IntoIterator<Item = Rule>) -> Self {
        Rule::Or(exprs.into_iter().collect())
    }

    pub fn and(exprs: impl IntoIterator<Item = Rule>) -> Self {
        Rule::And(exprs.into_iter().collect())
    }
}

impl Rule {
    /// Проверяет, подходит ли имя параметра под данное выражение
    pub fn matches(&self, name: &str) -> bool {
        match self {
            Rule::Name(n) => name == n,
            Rule::Not(expr) => !expr.matches(name),
            Rule::Or(exprs) => exprs.iter().any(|expr| expr.matches(name)),
            Rule::And(exprs) => exprs.iter().all(|expr| expr.matches(name)),
            Rule::All => true,
        }
    }
}
