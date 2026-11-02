#[derive(Debug, thiserror::Error)]
#[error("item is not mutated")]
pub(crate) struct ItemMutatedError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum MutatorInplaceError {
    #[error(transparent)]
    NotMutated(#[from] ItemMutatedError),
    #[error("mutated item type would change")]
    MutatedTypeChange,
}
