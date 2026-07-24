use crate::Metadata;
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

    #[builder(default = true)]
    parameters: bool,
    #[builder(default = true)]
    versions: bool,
    #[builder(default = true)]
    date: bool,
    #[builder(default, setter(strip_option))]
    separator: Option<&'a str>,
}

impl IntoAtoms<'_> for MetadataFormat<'_> {
    fn collect(self, atoms: &mut Atoms<'_>) {
        atoms.push_right(&self.metadata.name);
        if self.date && !self.metadata.dates.is_empty() {
            if let Some(date) = self.metadata.dates.last() {
                atoms.push_left(RichText::new(date.to_string()).weak());
            }
        }

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
        if self.versions && !self.metadata.versions.is_empty() {
            atoms.push_right(format!("[{}]", self.metadata.versions.iter().format("][")));
        }
    }
}

impl Display for MetadataFormat<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.date && !self.metadata.dates.is_empty() {
            if let Some(date) = self.metadata.dates.last() {
                write!(f, "{date}")?;
            }
            if let Some(separator) = self.separator {
                write!(f, "{separator}")?;
            }
        }
        write!(f, "{}", self.metadata.name)?;
        // if !self.metadata.name.is_empty()
        //     && let Some(separator) = self.separator
        // {
        //     f.write_str(separator)?;
        // }
        if self.parameters && !self.metadata.parameters.is_empty() {
            if let Some(separator) = self.separator {
                write!(f, "{separator}")?;
            }
            write!(f, "{{{}}}", self.metadata.parameters.iter().format("}{"))?;
        }
        if self.versions && !self.metadata.versions.is_empty() {
            if let Some(separator) = self.separator {
                write!(f, "{separator}")?;
            }
            write!(f, "[{}]", self.metadata.versions.iter().format("]["))?;
        }
        Ok(())
    }
}

/// Date
#[derive(Clone, Copy, Debug, Default)]
pub struct DatesFormat<'a> {
    separator: Option<&'a str>,
}

// impl<'a, T1, T2, T3> FormatBuilder<'a, (T1, T2, T3, (), ())> {
//     pub fn date_and_separator(
//         self,
//         date: bool,
//         separator: Option<&'a str>,
//     ) -> FormatBuilder<'a, (T1, T2, T3, (bool,), (Option<&'a str>,))> {
//         self.date(date, separator) = (date,);
//         self.pa = (separator,);
//         // self.mean(mean_and_standard_deviation.mean)
//         //     .standard_deviation(mean_and_standard_deviation.standard_deviation)
//         //     .relative(mean_and_standard_deviation.kind.is_relative())
//     }
// }
