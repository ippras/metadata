use crate::Metadata;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter, Result};
use typed_builder::TypedBuilder;

impl Metadata {
    pub fn format(&self) -> MetadataFormatBuilder<'_, ((&Metadata,), (), (), ())> {
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
    #[builder(default, setter(transform = |separator: Option<&'a str>| Some(DatesFormat { separator })))]
    date: Option<DatesFormat<'a>>,
    // #[builder(default, setter(strip_option))]
    // separator: Option<&'a str>,
}

impl Display for MetadataFormat<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.metadata.name)?;
        // if !self.metadata.name.is_empty()
        //     && let Some(separator) = self.separator
        // {
        //     f.write_str(separator)?;
        // }
        if self.parameters && !self.metadata.parameters.is_empty() {
            write!(f, "{{{}}}", self.metadata.parameters.iter().format(","))?;
        }
        if self.versions && !self.metadata.versions.is_empty() {
            write!(
                f,
                "[{}]",
                self.metadata
                    .versions
                    .iter()
                    .format_with(",", |version, f| f(&version))
            )?;
        }
        if let Some(date_format) = self.date
            && !self.metadata.dates.is_empty()
        {
            if let Some(separator) = date_format.separator {
                write!(f, "{separator}")?;
            }
            if let Some(date) = self.metadata.dates.last() {
                write!(f, "{date}")?;
            }
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
