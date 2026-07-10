use crate::err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError};

#[derive(thiserror::Error, Debug)]
pub enum AddProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
