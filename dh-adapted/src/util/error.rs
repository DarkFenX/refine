#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("handler does not support cache")]
    NoCacheSupport,
    #[cfg(feature = "json")]
    #[error("unable to open cache for reading: {0}")]
    RamJsonReadFailed(String),
    #[cfg(feature = "json")]
    #[error("unable to decompress cache: {0}")]
    RamJsonDecompFailed(String),
    #[cfg(feature = "json")]
    #[error("unable to parse cache data: {0}")]
    RamJsonParseFailed(String),
}
