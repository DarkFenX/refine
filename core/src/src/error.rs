#[derive(thiserror::Error, Debug)]
pub enum SrcInitError {
    #[error("failed to fetch EVE data: {0}")]
    EveDataFetchFailed(String),
    #[error("failed to clean EVE data: {0}")]
    EveDataCleanupFailed(String),
}
