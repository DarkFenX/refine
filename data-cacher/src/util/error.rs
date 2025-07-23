#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "json-zfile")]
    #[error("unable to open cache for reading: {0}")]
    RamJsonReadFailed(String),
    #[cfg(feature = "json-zfile")]
    #[error("unable to decompress cache: {0}")]
    RamJsonDecompFailed(String),
    #[cfg(feature = "json-zfile")]
    #[error("unable to parse cache data: {0}")]
    RamJsonParseFailed(String),
}
