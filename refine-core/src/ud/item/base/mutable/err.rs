#[derive(Debug, thiserror::Error)]
#[error("item is not mutated")]
pub(crate) struct ItemMutatedError;
