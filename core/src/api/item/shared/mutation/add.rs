use crate::err::basic::ItemNotMutatedError;

#[derive(thiserror::Error, Debug)]
pub enum AddMutationError {
    #[error("{0}")]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
