use crate::{Parameters, rule::Name};
use egui::{Atom, AtomExt, Atoms, IntoAtoms, RichText};
use std::fmt::{Debug, Display, Formatter, Result};

impl Parameters {
    pub fn format<'a>(&'a self) -> ParametersFormat<'a> {
        ParametersFormat::new(self)
    }
}

// Format parameters
#[derive(Clone, Debug)]
pub struct ParametersFormat<'a> {
    parameters: &'a Parameters,
    parameter_formats: Vec<ParameterFormat>,
}

impl<'a> ParametersFormat<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        Self {
            parameters,
            parameter_formats: Vec::new(),
        }
    }

    pub fn push(&mut self, parameter_format: ParameterFormat) {
        self.parameter_formats.push(parameter_format);
    }
}

impl Display for ParametersFormat<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for parameter_format in &self.parameter_formats {
            // Создаем итератор, который оставляет только подходящие под фильтр параметры
            let mut matched_parameters = self
                .parameters
                .iter()
                .filter(|parameter| parameter_format.rule.matches(&parameter.name));
            match parameter_format.value {
                Value::All => {
                    for parameter in matched_parameters {
                        write!(f, "{{{parameter}}}")?;
                    }
                }
                Value::First => {
                    if let Some(parameter) = matched_parameters.next() {
                        write!(f, "{{{parameter}}}")?;
                    }
                }
                Value::Last => {
                    if let Some(parameter) = matched_parameters.last() {
                        write!(f, "{{{parameter}}}")?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Parameter format
#[derive(Clone, Debug)]
pub struct ParameterFormat {
    pub rule: Name,
    pub value: Value,
}

impl ParameterFormat {
    pub fn new(rule: Name, value: Value) -> Self {
        Self { rule, value }
    }

    pub fn all(rule: Name) -> Self {
        Self {
            rule,
            value: Value::All,
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

//

/// Align
#[derive(Clone, Copy, Debug, Default)]
pub enum Align {
    Left,
    #[default]
    Right,
}

#[cfg(feature = "egui")]
impl IntoAtoms<'_> for ParametersFormat<'_> {
    fn collect(self, atoms: &mut Atoms<'_>) {
        for parameter_format in &self.parameter_formats {
            let mut matched_parameters = self
                .parameters
                .iter()
                .filter(|parameter| parameter_format.rule.matches(&parameter.name));
            match parameter_format.value {
                Value::All => {
                    for parameter in matched_parameters {
                        atoms.push_right(&parameter.name);
                    }
                }
                Value::First => {
                    if let Some(parameter) = matched_parameters.next() {
                        atoms.push_right(&parameter.name);
                    }
                }
                Value::Last => {
                    if let Some(parameter) = matched_parameters.last() {
                        atoms.push_right(&parameter.name);
                    }
                }
            }
        }

        // if self.dates && !self.metadata.dates.is_empty() {
        //     if let Some(date) = self.metadata.dates.last() {
        //         atoms.push_left(RichText::new(date.to_string()).weak());
        //     }
        // }

        // // Name
        // atoms.push_right(&self.metadata.name);
        // // Dates
        // if !self.metadata.dates.is_empty() {
        //     match self.date {
        //         Some(Value::All) => {
        //             for date in &self.metadata.dates {
        //                 atoms.push_left(RichText::new(date.to_string()).weak());
        //             }
        //         }
        //         Some(Value::First) => {
        //             if let Some(date) = self.metadata.dates.first() {
        //                 atoms.push_left(RichText::new(date.to_string()).weak());
        //             }
        //         }
        //         Some(Value::Last) => {
        //             if let Some(date) = self.metadata.dates.last() {
        //                 atoms.push_left(RichText::new(date.to_string()).weak());
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        // // Parameters
        // if self.parameters && !self.metadata.parameters.is_empty() {
        //     atoms.push_right(
        //         Atom::from(format!(
        //             "{{{}}}",
        //             self.metadata.parameters.iter().format("}{")
        //         ))
        //         .atom_shrink(true),
        //     );
        // } else {
        //     atoms.push_right(Atom::default().atom_shrink(true));
        // }
        // // Versions
        // if self.versions && !self.metadata.versions.is_empty() {
        //     atoms.push_right(format!("[{}]", self.metadata.versions.iter().format("][")));
        // }
    }
}
