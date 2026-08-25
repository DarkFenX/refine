use crate::err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError};

#[derive(Debug, thiserror::Error)]
pub enum ProjAddError {
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error(transparent)]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
