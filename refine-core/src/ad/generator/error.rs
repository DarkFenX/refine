use crate::{ad::err::ADataGeneratorCleanupError, ed::err::EveDataHandlerError};

#[derive(Debug, thiserror::Error)]
pub enum ADataGeneratorError {
    #[error("failed to fetch EVE data")]
    DataFetch(#[from] EveDataHandlerError),
    #[error("failed to clean EVE data")]
    Cleanup(#[from] ADataGeneratorCleanupError),
}
