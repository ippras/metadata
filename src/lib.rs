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

pub const NAME: &str = "name";
pub const DESCRIPTION: &str = "description";
pub const AUTHORS: &str = "authors";
pub const VERSION: &str = "version";
pub const DATE: &str = "date";
pub const DEFAULT_VERSION: &str = "0.0.0";
pub const DEFAULT_DATE: &str = "1970-01-01";

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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

impl From<Arc<BTreeMap<PlSmallStr, PlSmallStr>>> for Metadata {
    fn from(value: Arc<BTreeMap<PlSmallStr, PlSmallStr>>) -> Self {
        Self(
            value
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }
}

impl From<Metadata> for BTreeMap<PlSmallStr, PlSmallStr> {
    fn from(value: Metadata) -> Self {
        value
            .0
            .into_iter()
            .map(|(key, value)| (PlSmallStr::from_string(key), PlSmallStr::from_string(value)))
            .collect()
    }
}

/// MetaDataFrame
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetaDataFrame<M = Metadata, D = DataFrame> {
    pub meta: M,
    pub data: D,
}

impl<M, D> MetaDataFrame<M, D> {
    pub const fn new(meta: M, data: D) -> Self {
        Self { meta, data }
    }
}

impl Eq for MetaDataFrame {}

impl Hash for MetaDataFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.meta.hash(state);
        assert!(!self.data.should_rechunk());
        for series in self.data.iter() {
            for value in series.iter() {
                value.hash(state);
            }
        }
    }
}

impl PartialEq for MetaDataFrame {
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta && self.data.equals_missing(&other.data)
    }
}

#[cfg(feature = "egui")]
pub mod egui;
mod error;
#[cfg(feature = "ipc")]
mod ipc;
