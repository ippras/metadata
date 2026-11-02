//! Метаданные вместе с таблицей данных.

use crate::Metadata;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

/// Таблица данных с метаданными.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MetaDataFrame<M = Metadata, D = DataFrame> {
    /// Метаданные.
    pub meta: M,
    /// Данные.
    pub data: D,
}

impl<M, D> MetaDataFrame<M, D> {
    pub const fn new(meta: M, data: D) -> Self {
        Self { meta, data }
    }
}
