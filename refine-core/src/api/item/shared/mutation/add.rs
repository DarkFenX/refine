use crate::err::basic::ItemNotMutatedError;

#[derive(Debug, thiserror::Error)]
pub enum AddMutationError {
    #[error("{0}")]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
