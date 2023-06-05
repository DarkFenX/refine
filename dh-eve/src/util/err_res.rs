use std::{error, fmt, result};

#[derive(Debug)]
pub enum ErrorKind {
    #[cfg(any(feature = "phb-http", feature = "phb-file"))]
    /// Unable to parse data due to it being in unexpected format.
    ///
    /// Includes suffix of requested entity.
    PhbUnexpectedFsdTopEntity(String),
    #[cfg(feature = "phb-http")]
    /// HTTP handler cannot use passed URL as base.
    ///
    /// Includes passed URL and text description of failure.
    PhbHttpInvalidBaseUrl(String, String),
    #[cfg(feature = "phb-http")]
    /// HTTP handler is unable to join base URL and suffix.
    ///
    /// Includes suffix and text description of failure.
    PhbHttpSuffixJoinFailed(String, String),
    #[cfg(feature = "phb-http")]
    /// HTTP handler is unable to fetch data.
    ///
    /// Includes suffix and text description of failure.
    PhbHttpSuffixFetchFailed(String, String),
    #[cfg(feature = "phb-http")]
    /// HTTP handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    PhbHttpSuffixParseFailed(String, String),
    #[cfg(feature = "phb-file")]
    /// File handler is unable to read data.
    ///
    /// Includes suffix and text description of failure.
    PhbFileSuffixReadFailed(String, String),
    #[cfg(feature = "phb-file")]
    /// File handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    PhbFileSuffixParseFailed(String, String),
    #[cfg(feature = "phb-file")]
    /// File handler is unable to find client version in metadata.
    PhbFileNoClientBuild,
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
            #[cfg(any(feature = "phb-http", feature = "phb-file"))]
            ErrorKind::PhbUnexpectedFsdTopEntity(suffix) => {
                write!(
                    f,
                    "{suffix} FSD decomposition failed: highest-level entity is not a map"
                )
            }
            #[cfg(feature = "phb-http")]
            ErrorKind::PhbHttpInvalidBaseUrl(url, msg) => write!(f, "invalid base URL \"{url}\": {msg}"),
            #[cfg(feature = "phb-http")]
            ErrorKind::PhbHttpSuffixJoinFailed(suffix, msg) => {
                write!(f, "{suffix} is failed to be joined to base URL: {msg}")
            }
            #[cfg(feature = "phb-http")]
            ErrorKind::PhbHttpSuffixFetchFailed(suffix, msg) => write!(f, "{suffix} fetching failed: {msg}"),
            #[cfg(feature = "phb-http")]
            ErrorKind::PhbHttpSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "phb-file")]
            ErrorKind::PhbFileSuffixReadFailed(suffix, msg) => write!(f, "{suffix} reading failed: {msg}"),
            #[cfg(feature = "phb-file")]
            ErrorKind::PhbFileSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "phb-file")]
            ErrorKind::PhbFileNoClientBuild => write!(f, "unable to find client build field"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
