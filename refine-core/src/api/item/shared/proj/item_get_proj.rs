use crate::err::basic::{ItemFoundError, ProjFoundError};

#[derive(Debug, thiserror::Error)]
pub enum GetProjError {
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjectionNotFound(#[from] ProjFoundError),
}
