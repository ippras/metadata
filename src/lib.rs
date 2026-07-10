use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub const ID_SALT: &str = "Metadata";

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(pub BTreeMap<String, String>);

impl Metadata {
    pub fn new() -> Self {
        Self(BTreeMap::new())
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

pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[ftl!("en/main.ftl")];

    pub const RU: &[&str] = &[ftl!("ru/main.ftl")];
}

pub mod r#const;

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "polars")]
pub mod polars;

mod format;

#[cfg(test)]
mod test {
    use super::*;
    use crate::r#const::*;
    use jiff::civil::Date;
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
        meta.insert(VERSIONS.to_owned(), Version::new(0, 0, 1).to_string());
        meta.insert(DATES.to_owned(), Date::default().to_string());
        println!("meta: {}", meta.format().build());
        println!("meta: {}", meta.format().date(None).build());
        println!("meta: {}", meta.format().date(Some(" ")).build());
        println!("meta: {}", meta.format().date(Some(".")).build());
    }
}
