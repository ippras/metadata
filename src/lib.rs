#![feature(debug_closure_helpers)]

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter, from_fn},
    ops::{Deref, DerefMut},
};

pub const ID_SALT: &str = "Metadata";

pub const AUTHORS: &str = "Authors";
pub const DATE_TIME: &str = "DateTime";
pub const DATE: &str = "Date";
pub const DESCRIPTION: &str = "Description";
pub const NAME: &str = "Name";
pub const PARAMETERS: &str = "Parameters";
pub const VERSION: &str = "Version";

pub const DEFAULT_DATE: &str = "1970-01-01";
pub const DEFAULT_VERSION: &str = "0.0.0";

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(pub BTreeMap<String, String>);

impl Metadata {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl Metadata {
    pub fn format(&self, separator: &str) -> impl Debug + Display {
        from_fn(move |f| {
            if let Some(name) = self.get(NAME) {
                write!(f, "{name}")?;
            }
            if let Some(parameters) = self.get(PARAMETERS)
                && !parameters.is_empty()
            {
                write!(f, "{{{parameters}}}")?;
            }
            if let Some(version) = self.get(VERSION)
                && version != DEFAULT_VERSION
            {
                write!(f, "[{}]", version.trim_start_matches(['0', '.']))?;
            }
            if let Some(date) = self.get(DATE)
                && date != DEFAULT_DATE
            {
                write!(f, "{separator}{date}")?;
            }
            Ok(())
        })
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(name) = self.get(NAME) {
            write!(f, "{name}")?;
        }
        if let Some(parameters) = self.get(PARAMETERS)
            && !parameters.is_empty()
        {
            write!(f, "{{{parameters}}}")?;
        }
        if let Some(version) = self.get(VERSION)
            && version != DEFAULT_VERSION
        {
            write!(f, "[{}]", version.trim_start_matches(['0', '.']))?;
        }
        if let Some(date) = self.get(DATE)
            && date != DEFAULT_DATE
        {
            write!(f, "{date}")?;
        }
        Ok(())
    }
}

impl Deref for Metadata {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(String, String)> for Metadata {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Self(BTreeMap::from_iter(iter))
    }
}

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "polars")]
pub mod polars;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use semver::Version;

    #[test]
    fn test() {
        let mut meta = Metadata::default();
        meta.insert(NAME.to_owned(), "The name".to_owned());
        meta.insert(DESCRIPTION.to_owned(), "The description".to_owned());
        meta.insert(
            AUTHORS.to_owned(),
            "Giorgi Vladimirovich Kazakov;Roman Alexandrovich Sidorov".to_owned(),
        );
        meta.insert(
            PARAMETERS.to_owned(),
            format!("InitialTemperature={};TemperatureStep={}", 0, 1),
        );
        meta.insert(VERSION.to_owned(), Version::new(0, 0, 1).to_string());
        meta.insert(DATE.to_owned(), NaiveDate::default().to_string());
        println!("meta: {meta}");
    }
}
