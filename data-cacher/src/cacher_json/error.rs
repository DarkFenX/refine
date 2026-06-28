#[derive(thiserror::Error, Debug)]
pub enum JsonZfileAdcError {
    #[error("unable to open cache for reading: {0}")]
    ReadFailed(String),
    #[error("unable to decompress cache: {0}")]
    DecompFailed(String),
    #[error("unable to parse cache data: {0}")]
    ParseFailed(String),
}
