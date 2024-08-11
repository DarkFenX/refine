#[derive(Debug)]
pub enum SrcInitError {
    EveDataFetchFailed(String),
    EveDataCleanupFailed(String),
}
impl std::error::Error for SrcInitError {}
impl std::fmt::Display for SrcInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EveDataFetchFailed(msg) => write!(f, "failed to fetch EVE data: {msg}"),
            Self::EveDataCleanupFailed(msg) => write!(f, "failed to clean EVE data: {msg}"),
        }
    }
}
