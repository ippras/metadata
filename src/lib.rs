#![feature(debug_closure_helpers)]

// pub use self::error::{Error, Result};

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, from_fn},
    ops::{Deref, DerefMut},
};

pub const AUTHORS: &str = "Authors";
pub const DATE: &str = "Date";
pub const DESCRIPTION: &str = "Description";
pub const NAME: &str = "Name";
pub const VERSION: &str = "Version";

pub const DEFAULT_DATE: &str = "1970-01-01";
pub const DEFAULT_VERSION: &str = "0.0.0";

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata(pub BTreeMap<String, String>);

impl Metadata {
    pub fn format(&self, separator: &str) -> impl Display {
        from_fn(move |f| {
            if let Some(name) = self.get(NAME) {
                write!(f, "{name}")?;
            }
            if let Some(date) = self.get(DATE)
                && date != DEFAULT_DATE
            {
                write!(f, "{separator}{date}")?;
            }
            if let Some(version) = self.get(VERSION)
                && version != DEFAULT_VERSION
            {
                write!(f, "{separator}{version}")?;
            }
            Ok(())
        })
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
// mod error;
#[cfg(feature = "polars")]
pub mod polars;
