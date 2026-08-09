use crate::err::basic::ItemNotMutatedError;

#[derive(Debug, thiserror::Error)]
pub enum AddMutationError {
    #[error(transparent)]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
