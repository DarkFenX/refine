use crate::err::basic::{ItemFoundError, ProjFoundError};

#[derive(Debug, thiserror::Error)]
pub enum GetProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
