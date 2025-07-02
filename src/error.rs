use std::io;

use polars::prelude::PolarsError;
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error
#[derive(Error, Debug)]
pub enum Error {
    #[error(r#"metadata date "{0}""#)]
    Date(#[from] chrono::ParseError),
    #[error(r#"metadata version "{0}""#)]
    Version(#[from] semver::Error),
    #[error(r#"polars metadata "{0}""#)]
    Polars(#[from] PolarsError),
    #[error(r#"io "{0}""#)]
    Io(#[from] io::Error),
}
