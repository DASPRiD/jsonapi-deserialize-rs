use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Document returned errors")]
    DocumentError(Vec<crate::document::DocumentError>),

    #[error(transparent)]
    DeserializeError(#[from] crate::deserialize::Error),
}
