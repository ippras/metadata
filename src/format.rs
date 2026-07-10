use crate::{
    Metadata,
    r#const::{DATES, NAME, PARAMETERS, VERSIONS},
};
use std::fmt::{Debug, Display, Formatter, Result};
use typed_builder::TypedBuilder;

impl Metadata {
    pub fn format(&self) -> FormatBuilder<'_, ((&Metadata,), (), (), ())> {
        Format::builder().metadata(self)
    }
}

// Format metadata
#[derive(Clone, Copy, Debug, TypedBuilder)]
pub struct Format<'a> {
    metadata: &'a Metadata,

    #[builder(default = true)]
    parameters: bool,
    #[builder(default = true)]
    version: bool,
    #[builder(default, setter(transform = |separator: Option<&'a str>| Some(Date { separator })))]
    date: Option<Date<'a>>,
}

impl Display for Format<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(value) = self.metadata.get(NAME) {
            write!(f, "{value}")?;
        }
        if self.parameters
            && let Some(value) = self.metadata.get(PARAMETERS)
        {
            write!(f, "{{{value}}}")?;
        }
        if self.version
            && let Some(value) = self.metadata.get(VERSIONS)
        {
            write!(f, "[{}]", value.trim_start_matches("0."))?;
        }
        if let Some(date) = self.date
            && let Some(value) = self.metadata.get(DATES)
        {
            if let Some(separator) = date.separator {
                write!(f, "{separator}")?;
            }
            write!(f, "{value}")?;
        }
        Ok(())
    }
}

/// Date
#[derive(Clone, Copy, Debug, Default)]
pub struct Date<'a> {
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

// impl Metadata {
//     pub fn format(&self, separator: &str) -> impl Debug + Display {
//         from_fn(move |f| {
//             if let Some(name) = self.get(NAME) {
//                 write!(f, "{name}")?;
//             }
//             if let Some(parameters) = self.get(PARAMETERS)
//                 && !parameters.is_empty()
//             {
//                 write!(f, "{{{parameters}}}")?;
//             }
//             if let Some(version) = self.get(VERSION)
//                 && version != DEFAULT_VERSION
//             {
//                 write!(f, "[{}]", version.trim_start_matches(['0', '.']))?;
//             }
//             if let Some(date) = self.get(DATE)
//                 && date != DEFAULT_DATE
//             {
//                 write!(f, "{separator}{date}")?;
//             }
//             Ok(())
//         })
//     }
// }

// impl Display for Metadata {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         if let Some(name) = self.get(NAME) {
//             write!(f, "{name}")?;
//         }
//         if let Some(parameters) = self.get(PARAMETERS)
//             && !parameters.is_empty()
//         {
//             write!(f, "{{{parameters}}}")?;
//         }
//         if let Some(version) = self.get(VERSION)
//             && version != DEFAULT_VERSION
//         {
//             write!(f, "[{}]", version.trim_start_matches(['0', '.']))?;
//         }
//         if let Some(date) = self.get(DATE)
//             && date != DEFAULT_DATE
//         {
//             write!(f, "{date}")?;
//         }
//         Ok(())
//     }
// }
