use crate::r#const::EQUAL;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Параметр метаданных: пара «имя — значение».
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Parameter<T = String, U = String> {
    pub name: T,
    pub value: U,
}

impl Parameter {
    /// Создаёт параметр из любых значений, конвертируемых в `String`.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.name, EQUAL, self.value)
    }
}
