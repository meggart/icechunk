use icechunk::{
    format::IcechunkFormatError, repository::RepositoryError, zarr::StoreError,
};
use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    PyErr,
};
use thiserror::Error;

/// A simple wrapper around the StoreError to make it easier to convert to a PyErr
///
/// When you use the ? operator, the error is coerced. But if you return the value it is not.
/// So for now we just use the extra operation to get the coercion instead of manually mapping
/// the errors where this is returned from a python class
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
#[allow(dead_code)]
pub(crate) enum PyIcechunkStoreError {
    #[error("store error: {0}")]
    StoreError(#[from] StoreError),
    #[error("repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("icechunk format error: {0}")]
    IcechunkFormatError(#[from] IcechunkFormatError),
    #[error("{0}")]
    PyKeyError(String),
    #[error("{0}")]
    PyValueError(String),
    #[error("{0}")]
    PyError(#[from] PyErr),
    #[error("{0}")]
    UnkownError(String),
}

impl From<PyIcechunkStoreError> for PyErr {
    fn from(error: PyIcechunkStoreError) -> Self {
        match error {
            PyIcechunkStoreError::PyKeyError(e) => PyKeyError::new_err(e),
            PyIcechunkStoreError::PyValueError(e) => PyValueError::new_err(e),
            PyIcechunkStoreError::PyError(err) => err,
            _ => PyValueError::new_err(error.to_string()),
        }
    }
}

pub(crate) type PyIcechunkStoreResult<T> = Result<T, PyIcechunkStoreError>;
