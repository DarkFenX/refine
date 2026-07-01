use crate::ad::ADataGeneratorError;

#[derive(thiserror::Error, Debug)]
pub enum SrcInitError {
    #[error("failed to fetch EVE data: {0}")]
    EveDataFetchFailed(String),
    #[error("failed to clean EVE data: {0}")]
    EveDataCleanupFailed(String),
}
impl From<ADataGeneratorError> for SrcInitError {
    fn from(error: ADataGeneratorError) -> Self {
        match error {
            ADataGeneratorError::DataFetchFailed(error) => SrcInitError::EveDataFetchFailed(error),
            ADataGeneratorError::CleanupFailed(error) => SrcInitError::EveDataCleanupFailed(error),
        }
    }
}
