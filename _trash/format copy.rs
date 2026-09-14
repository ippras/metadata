use crate::{
    Metadata,
    r#const::{DATE, NAME, VERSION},
};
use egui::{Atom, AtomExt, Atoms, IntoAtoms, RichText};
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter, Result};
use typed_builder::TypedBuilder;

impl Metadata {
    pub fn format(&self) -> MetadataFormatBuilder<'_, ((&Metadata,), (), (), (), ())> {
        MetadataFormat::builder().metadata(self)
    }
}

// Format metadata
#[derive(Clone, Copy, Debug, TypedBuilder)]
pub struct MetadataFormat<'a> {
    metadata: &'a Metadata,

    #[builder(default = Some(Many::Last), setter(strip_option))]
    dates: Option<Many>,
    #[builder(default = true)]
    name: bool,
    #[builder(default = true)]
    versions: bool,
    #[builder(default = true)]
    parameters: bool,
}

impl Display for MetadataFormat<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Dates
        if !self.metadata.dates.is_empty() {
            match self.dates {
                Some(Many::All) => {
                    for date in &self.metadata.dates {
                        write!(f, "{{{DATE}={date}}}")?;
                    }
                }
                Some(Many::First) => {
                    if let Some(date) = self.metadata.dates.first() {
                        write!(f, "{{{DATE}={date}}}")?;
                    }
                }
                Some(Many::Last) => {
                    if let Some(date) = self.metadata.dates.last() {
                        write!(f, "{{{DATE}={date}}}")?;
                    }
                }
                _ => {}
            }
        }
        // Name
        if self.name {
            write!(f, "{{{NAME}={}}}", self.metadata.name)?;
        }
        // Parameters
        if self.parameters && !self.metadata.parameters.is_empty() {
            write!(f, "{{{}}}", self.metadata.parameters.iter().format("}{"))?;
        }
        // Versions
        if self.versions {
            for version in &self.metadata.versions {
                write!(f, "{{{VERSION}={version}}}")?;
            }
        }
        Ok(())
    }
}

/// Many
#[derive(Clone, Copy, Debug)]
pub enum Many {
    All,
    First,
    Last,
}

/// Key
#[derive(Clone, Copy, Debug)]
pub enum Key {
    Date,
    Name,
    Parameter,
    Version,
}

#[cfg(feature = "egui")]
impl IntoAtoms<'_> for MetadataFormat<'_> {
    fn collect(self, atoms: &mut Atoms<'_>) {
        // if self.dates && !self.metadata.dates.is_empty() {
        //     if let Some(date) = self.metadata.dates.last() {
        //         atoms.push_left(RichText::new(date.to_string()).weak());
        //     }
        // }

        // Name
        atoms.push_right(&self.metadata.name);
        // Dates
        if !self.metadata.dates.is_empty() {
            match self.dates {
                Some(Many::All) => {
                    for date in &self.metadata.dates {
                        atoms.push_left(RichText::new(date.to_string()).weak());
                    }
                }
                Some(Many::First) => {
                    if let Some(date) = self.metadata.dates.first() {
                        atoms.push_left(RichText::new(date.to_string()).weak());
                    }
                }
                Some(Many::Last) => {
                    if let Some(date) = self.metadata.dates.last() {
                        atoms.push_left(RichText::new(date.to_string()).weak());
                    }
                }
                _ => {}
            }
        }
        // Parameters
        if self.parameters && !self.metadata.parameters.is_empty() {
            atoms.push_right(
                Atom::from(format!(
                    "{{{}}}",
                    self.metadata.parameters.iter().format("}{")
                ))
                .atom_shrink(true),
            );
        } else {
            atoms.push_right(Atom::default().atom_shrink(true));
        }
        // Versions
        if self.versions && !self.metadata.versions.is_empty() {
            atoms.push_right(format!("[{}]", self.metadata.versions.iter().format("][")));
        }
    }
}
