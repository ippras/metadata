#![feature(debug_closure_helpers)]

pub use self::error::{Error, Result};

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, from_fn},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

pub const AUTHORS: &str = "authors";
pub const DATE: &str = "date";
pub const DESCRIPTION: &str = "description";
pub const NAME: &str = "name";
pub const VERSION: &str = "version";

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

    pub fn clear_schema(&mut self) {
        self.retain(|key, _| key != "ARROW:schema");
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

// impl FromIterator for Metadata

/// MetaDataFrame
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MetaDataFrame<M = Metadata, D = DataFrame> {
    pub meta: M,
    pub data: D,
}

impl<M, D> MetaDataFrame<M, D> {
    pub const fn new(meta: M, data: D) -> Self {
        Self { meta, data }
    }
}

// impl<M: PartialEq> Eq for MetaDataFrame<M> {}

// impl Hash for MetaDataFrame<Metadata, DataFrame> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.meta.hash(state);
//         assert!(!self.data.should_rechunk());
//         for series in self.data.iter() {
//             for value in series.iter() {
//                 value.hash(state);
//             }
//         }
//     }
// }

// impl<M: PartialEq> PartialEq for MetaDataFrame<M> {
//     fn eq(&self, other: &Self) -> bool {
//         self.meta == other.meta && self.data.equals_missing(&other.data)
//     }
// }

#[cfg(feature = "egui")]
pub mod egui;
mod error;
// #[cfg(feature = "ipc")]
// mod ipc;
#[cfg(feature = "parquet")]
mod parquet;
