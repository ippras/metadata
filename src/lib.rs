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

use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    ops::{Deref, DerefMut},
};

use crate::r#const::{AUTHORS, DATES, DESCRIPTION, NAME, PARAMETERS, VERSIONS};

pub const ID_SALT: &str = "Metadata";

// /// Metadata
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct Metadata(pub BTreeMap<String, String>);

// impl Metadata {
//     pub fn new() -> Self {
//         Self(BTreeMap::new())
//     }
// }

// impl Deref for Metadata {
//     type Target = BTreeMap<String, String>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Metadata {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl FromIterator<(String, String)> for Metadata {
//     fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
//         Self(BTreeMap::from_iter(iter))
//     }
// }

// /// Metadata value
// #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub enum Value {
//     Name(String),
//     Description(String),
//     Authors(Vec<String>),
//     Parameters(Vec<Parameter>),
//     Versions(Vec<Version>),
//     Dates(Vec<Date>),
// }

// /// Metadata
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct AMetadata(pub BTreeMap<String, Value>);

// impl AMetadata {
//     pub fn new() -> Self {
//         Self(BTreeMap::new())
//     }
// }

// impl Deref for AMetadata {
//     type Target = BTreeMap<String, Value>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for AMetadata {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl FromIterator<(String, Value)> for AMetadata {
//     fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
//         Self(BTreeMap::from_iter(iter))
//     }
// }

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameters: Vec<Parameter>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub versions: Vec<Version>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
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
    pub key: String,
    pub value: Option<String>,
}

impl Parameter {
    fn new() -> Self {
        Self {
            key: String::new(),
            value: None,
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        if let Some(value) = &self.value {
            write!(f, "={value}")?;
        }
        Ok(())
    }
}

/// Version
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
        println!("meta: {}", meta.format().build());
        println!("meta: {}", meta.format().dates(None).build());
        println!("meta: {}", meta.format().dates(Some(" ")).build());
        println!("meta: {}", meta.format().dates(Some(".")).build());
    }
}
