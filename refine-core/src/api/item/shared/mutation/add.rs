use crate::err::basic::ItemNotMutatedError;

#[derive(Debug, thiserror::Error)]
pub enum MutationAddError {
    #[error(transparent)]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
