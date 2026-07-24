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

use crate::r#const::{AUTHORS, DATES, DESCRIPTION, NAME, PARAMETERS, VERSIONS};
use jiff::civil::Date;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    sync::LazyLock,
};

pub const ID_SALT: &str = "Metadata";

pub static PRETTY_CONFIG: LazyLock<PrettyConfig> = LazyLock::new(|| {
    PrettyConfig::new()
        .depth_limit(2)
        .extensions(Extensions::UNWRAP_NEWTYPES | Extensions::IMPLICIT_SOME)
        .new_line("\n")
});

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<Version>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dates: Vec<Date>,
}

impl TryFrom<Metadata> for BTreeMap<String, String> {
    type Error = ron::Error;

    fn try_from(value: Metadata) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        if !value.name.is_empty() {
            map.insert(NAME.to_owned(), value.name);
        }
        if !value.description.is_empty() {
            map.insert(DESCRIPTION.to_owned(), value.description);
        }
        if !value.authors.is_empty() {
            map.insert(AUTHORS.to_owned(), ron::ser::to_string(&value.authors)?);
        }
        if !value.parameters.is_empty() {
            map.insert(
                PARAMETERS.to_owned(),
                ron::ser::to_string(&value.parameters)?,
            );
        }
        if !value.versions.is_empty() {
            map.insert(VERSIONS.to_owned(), ron::ser::to_string(&value.versions)?);
        }
        if !value.dates.is_empty() {
            map.insert(DATES.to_owned(), ron::ser::to_string(&value.dates)?);
        }
        Ok(map)
    }
}

/// Version
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Parameter {
    fn new() -> Self {
        Self {
            name: String::new(),
            value: None,
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value) = &self.value {
            write!(f, "={value}")?;
        }
        Ok(())
    }
}

/// Version
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Version(u64, u64, u64);

impl Version {
    fn new() -> Self {
        Self(0, 0, 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.0 != 0 {
            write!(f, "{}", self.0)?;
        }
        if self.1 != 0 {
            if self.0 != 0 {
                f.write_str(".")?;
            }
            write!(f, "{}", self.1)?;
        }
        if self.0 != 0 || self.1 != 0 {
            f.write_str(".")?;
        }
        write!(f, "{}", self.2)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::r#const::*;
    use jiff::civil::Date;
    use ron::{extensions::Extensions, ser::PrettyConfig};
    use semver::Version;

    #[test]
    fn test() {
        // let mut meta = AMetadata(BTreeMap::new());
        // meta.insert(NAME.to_owned(), Value::Name("VIR-2699".to_owned()));
        // meta.insert(
        //     DESCRIPTION.to_owned(),
        //     Value::Description("Cat. No. 2699, Прогресс, Россия".to_owned()),
        // );
        // meta.insert(
        //     AUTHORS.to_owned(),
        //     Value::Authors(vec![
        //         "Giorgi Vladimirovich Kazakov".to_owned(),
        //         "Roman Alexandrovich Sidorov".to_owned(),
        //     ]),
        // );
        let mut meta = Metadata {
            name: "VIR-2699".to_owned(),
            description: "Cat. No. 2699, Прогресс, Россия".to_owned(),
            authors: vec![
                "Giorgi Vladimirovich Kazakov".to_owned(),
                "Roman Alexandrovich Sidorov".to_owned(),
            ],
            parameters: Vec::new(),
            versions: Vec::new(),
            dates: Vec::new(),
        };
        println!("meta: {meta:?}");
        let contents = ron::ser::to_string_pretty(
            &meta,
            PrettyConfig::new()
                .depth_limit(2)
                .extensions(Extensions::UNWRAP_NEWTYPES)
                .new_line("\n"),
        )
        .unwrap();
        std::fs::write("path.ron", &contents).unwrap();
        println!("meta: {contents:?}");

        let mut meta = Metadata::default();
        // meta.insert(NAME.to_owned(), "The name".to_owned());
        // meta.insert(DESCRIPTION.to_owned(), "The description".to_owned());
        // meta.insert(
        //     AUTHORS.to_owned(),
        //     "Giorgi Vladimirovich Kazakov;Roman Alexandrovich Sidorov".to_owned(),
        // );
        // meta.insert(
        //     PARAMETERS.to_owned(),
        //     format!("InitialTemperature={};TemperatureStep={}", 0, 1),
        // );
        // meta.insert(VERSIONS.to_owned(), Version::new(0, 0, 1).to_string());
        // meta.insert(DATES.to_owned(), Date::default().to_string());
        println!("meta: {}", meta.format().date(false).build());
        println!("meta: {}", meta.format().build());
        println!("meta: {}", meta.format().separator(" ").build());
        println!("meta: {}", meta.format().separator(".").build());
    }
}
