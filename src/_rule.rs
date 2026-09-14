/// Rule
#[derive(Clone, Debug)]
pub enum Rule {
    /// Точное совпадение параметра
    NameValue { name: String, value: Value },
    /// Отрицание (NOT): всё, кроме указанных в списке
    Not(Box<Self>),
    /// Логическое ИЛИ для сложных вложенных правил
    Or(Vec<Rule>),
    /// Логическое И для сложных вложенных правил
    And(Vec<Rule>),
    /// Захватить вообще всё
    All { value: Value },
}

impl Rule {
    pub fn name(name: impl Into<String>) -> Self {
        Rule::NameValue {
            name: name.into(),
            value: Value::All,
        }
    }

    pub fn name_value(name: impl Into<String>, value: Value) -> Self {
        Rule::NameValue {
            name: name.into(),
            value,
        }
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

    pub fn all() -> Self {
        Rule::All { value: Value::All }
    }
}

impl Rule {
    /// Проверяет, подходит ли имя параметра под данное выражение
    pub fn matches(&self, name: &str) -> Option<Value> {
        match self {
            Rule::NameValue {
                name: target,
                value,
            } => {
                if name == target {
                    Some(*value)
                } else {
                    None
                }
            }
            Rule::Not(rule) => {
                if rule.matches(name).is_none() {
                    Some(Value::All)
                } else {
                    None
                }
            }
            Rule::Or(rules) => rules.iter().find_map(|rule| rule.matches(name)),
            Rule::And(rules) => {
                let mut matched = true;
                let mut final_policy = Value::All;

                for rule in rules {
                    if let Some(value) = rule.matches(name) {
                        // Если внутри And есть специфичная политика, запоминаем её
                        if value != Value::All {
                            final_policy = value;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched { Some(final_policy) } else { None }
            }
            Rule::All { value } => Some(*value),
        }
    }
}

/// Value
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Value {
    #[default]
    All,
    First,
    Last,
}
