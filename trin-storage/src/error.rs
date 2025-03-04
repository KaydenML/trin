use ethportal_api::{
    types::{content_key::error::ContentKeyError, distance::Distance},
    utils::bytes::ByteUtilsError,
};
use thiserror::Error;

/// An error from an operation on a `ContentStore`.
#[derive(Debug, Error)]
pub enum ContentStoreError {
    #[error("An error from the underlying database: {0:?}")]
    Database(String),

    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),

    /// Unable to store content because it does not fall within the store's radius.
    #[error("radius {radius} insufficient to store content at distance {distance}")]
    InsufficientRadius {
        radius: Distance,
        distance: Distance,
    },

    /// Unable to store or retrieve data because it is invalid.
    #[error("data invalid {message}")]
    InvalidData { message: String },

    #[error("rusqlite error {0}")]
    Rusqlite(#[from] rusqlite::Error),

    #[error("r2d2 error {0}")]
    R2D2(#[from] r2d2::Error),

    #[error("unable to use byte utils {0}")]
    ByteUtilsError(#[from] ByteUtilsError),

    #[error("unable to use content key {0}")]
    ContentKey(#[from] ContentKeyError),
}
