#[derive(Debug, thiserror::Error)]
pub(crate) enum ADataGeneratorError {
    #[error("failed to fetch data: {0}")]
    DataFetchFailed(String),
    #[error("failed to clean data: {0}")]
    CleanupFailed(String),
}
