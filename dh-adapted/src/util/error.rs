#[derive(Debug)]
pub enum Error {
    NoCacheSupport,
    #[cfg(feature = "json")]
    RamJsonReadFailed(String),
    #[cfg(feature = "json")]
    RamJsonDecompFailed(String),
    #[cfg(feature = "json")]
    RamJsonParseFailed(String),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoCacheSupport => write!(f, "handler does not support cache"),
            #[cfg(feature = "json")]
            Self::RamJsonReadFailed(msg) => write!(f, "unable to open cache for reading: {msg}"),
            #[cfg(feature = "json")]
            Self::RamJsonDecompFailed(msg) => write!(f, "unable to decompress cache: {msg}"),
            #[cfg(feature = "json")]
            Self::RamJsonParseFailed(msg) => write!(f, "unable to parse cache data: {msg}"),
        }
    }
}
