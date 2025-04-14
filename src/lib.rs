pub use self::error::{Error, Result};

use chrono::NaiveDate;
use polars::{io::mmap::MmapBytesReader, prelude::*};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    collections::BTreeMap,
    hash::{Hash, Hasher},
    io::Write,
};

pub const DATE_FORMAT: &str = "%Y-%m-%d";

/// Metadata
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub version: Option<Version>,
    pub date: Option<NaiveDate>,
}

impl Metadata {
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            authors: Vec::new(),
            version: None,
            date: None,
        }
    }

    pub fn title(&self) -> String {
        match &self.version {
            Some(version) => format!("{} {version}", self.name),
            None => self.name.to_owned(),
        }
    }
}

impl TryFrom<&BTreeMap<PlSmallStr, PlSmallStr>> for Metadata {
    type Error = Error;

    fn try_from(value: &BTreeMap<PlSmallStr, PlSmallStr>) -> Result<Self> {
        Ok(Self {
            name: value
                .get("name")
                .map_or_else(String::new, ToString::to_string),
            description: value
                .get("description")
                .map_or_else(String::new, ToString::to_string),
            authors: value.get("authors").map_or_else(Vec::new, |authors| {
                authors.split(",").map(ToOwned::to_owned).collect()
            }),
            version: value
                .get("version")
                .map(|version| Version::parse(version))
                .transpose()?,
            date: value
                .get("date")
                .map(|date| NaiveDate::parse_from_str(date, DATE_FORMAT))
                .transpose()?,
        })
    }
}

impl From<Metadata> for BTreeMap<PlSmallStr, PlSmallStr> {
    fn from(value: Metadata) -> Self {
        let mut metadata = BTreeMap::new();
        metadata.insert("name".into(), value.name.into());
        metadata.insert("description".into(), value.description.into());
        if !value.authors.is_empty() {
            metadata.insert("authors".into(), value.authors.join(",").into());
        }
        if let Some(version) = value.version {
            metadata.insert("version".into(), version.to_string().into());
        }
        if let Some(date) = value.date {
            metadata.insert("date".into(), date.format(DATE_FORMAT).to_string().into());
        }
        metadata
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

impl MetaDataFrame {
    pub fn read(reader: impl MmapBytesReader) -> Result<Self> {
        let mut reader = IpcReader::new(reader);
        let meta = reader.metadata()?.unwrap_or_default();
        let data = reader.finish()?;
        Ok(Self { meta, data })
    }
}

impl<D: BorrowMut<DataFrame>> MetaDataFrame<Metadata, D> {
    pub fn write(mut self, writer: impl Write) -> Result<()> {
        let mut writer = IpcWriter::new(writer);
        writer.metadata(self.meta);
        writer.finish(self.data.borrow_mut())?;
        Ok(())
    }
}

impl Eq for MetaDataFrame {}

impl Hash for MetaDataFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.meta.hash(state);
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

/// Extension methods for [`IpcReader`]
pub trait IpcReaderExt {
    fn metadata(&mut self) -> Result<Option<Metadata>>;
}

impl<R: MmapBytesReader> IpcReaderExt for IpcReader<R> {
    fn metadata(&mut self) -> Result<Option<Metadata>> {
        let Some(metadata) = self.custom_metadata()? else {
            return Ok(None);
        };
        let metadata = Metadata::try_from(&*metadata)?;
        Ok(Some(metadata))
    }
}

/// Extension methods for [`IpcWriter`]
pub trait IpcWriterExt {
    fn metadata(&mut self, metadata: Metadata);
}

impl<W: Write> IpcWriterExt for IpcWriter<W> {
    fn metadata(&mut self, metadata: Metadata) {
        self.set_custom_schema_metadata(Arc::new(metadata.into()));
    }
}

#[cfg(feature = "egui")]
pub mod egui;
mod error;
