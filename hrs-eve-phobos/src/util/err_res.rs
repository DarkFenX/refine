use std::{error, fmt, result};

#[derive(Debug)]
pub enum ErrorKind {
    /// Unable to parse data due to it being in unexpected format.
    ///
    /// Includes suffix of requested entity.
    UnexpectedFsdTopEntity(String),
    #[cfg(feature = "http")]
    /// HTTP handler cannot use passed URL as base.
    ///
    /// Includes passed URL and text description of failure.
    HttpInvalidBaseUrl(String, String),
    #[cfg(feature = "http")]
    /// HTTP handler is unable to join base URL and suffix.
    ///
    /// Includes suffix and text description of failure.
    HttpSuffixJoinFailed(String, String),
    #[cfg(feature = "http")]
    /// HTTP handler is unable to fetch data.
    ///
    /// Includes suffix and text description of failure.
    HttpSuffixFetchFailed(String, String),
    #[cfg(feature = "http")]
    /// HTTP handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    HttpSuffixParseFailed(String, String),
    #[cfg(feature = "file")]
    /// File handler is unable to read data.
    ///
    /// Includes suffix and text description of failure.
    FileSuffixReadFailed(String, String),
    #[cfg(feature = "file")]
    /// File handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    FileSuffixParseFailed(String, String),
    #[cfg(feature = "file")]
    /// File handler is unable to find client version in metadata.
    FileNoClientBuild,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedFsdTopEntity(suffix) => {
                write!(
                    f,
                    "{suffix} FSD decomposition failed: highest-level entity is not a map"
                )
            }
            #[cfg(feature = "http")]
            ErrorKind::HttpInvalidBaseUrl(url, msg) => write!(f, "invalid base URL \"{url}\": {msg}"),
            #[cfg(feature = "http")]
            ErrorKind::HttpSuffixJoinFailed(suffix, msg) => {
                write!(f, "{suffix} is failed to be joined to base URL: {msg}")
            }
            #[cfg(feature = "http")]
            ErrorKind::HttpSuffixFetchFailed(suffix, msg) => write!(f, "{suffix} fetching failed: {msg}"),
            #[cfg(feature = "http")]
            ErrorKind::HttpSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "file")]
            ErrorKind::FileSuffixReadFailed(suffix, msg) => write!(f, "{suffix} reading failed: {msg}"),
            #[cfg(feature = "file")]
            ErrorKind::FileSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "file")]
            ErrorKind::FileNoClientBuild => write!(f, "unable to find client build field"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
