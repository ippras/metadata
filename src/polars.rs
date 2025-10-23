use crate::Metadata;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

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
