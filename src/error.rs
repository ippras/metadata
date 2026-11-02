use std::io;
use thiserror::Error;

/// Результат операций крейта.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Ошибки крейта.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    // /// Некорректная дата метаданных.
    // #[error("invalid metadata date: {0}")]
    // Date(#[from] chrono::ParseError),
    /// Некорректная версия метаданных.
    #[error("invalid metadata version: {0}")]
    Version(#[from] semver::Error),
    /// Ошибка ввода-вывода.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
