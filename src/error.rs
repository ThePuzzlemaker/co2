//! Error types and helpers
use thiserror::Error;

/// An enum describing the errors that can occur in CO2
#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum CO2Error {
    /// An arbitrary message
    #[error("{0}")]
    Msg(String),
    /// Some other error, via [`anyhow`]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// A convenient type alias for a [`Result`] with an error type of [`CO2Error`]
pub type CO2Result<T> = Result<T, CO2Error>;
