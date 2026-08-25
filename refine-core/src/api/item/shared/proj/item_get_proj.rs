use crate::err::basic::{ItemFoundError, ProjFoundError};

#[derive(Debug, thiserror::Error)]
pub enum ProjGetError {
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjectionNotFound(#[from] ProjFoundError),
}
