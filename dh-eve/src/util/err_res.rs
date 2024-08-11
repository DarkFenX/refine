#[derive(Debug)]
pub enum Error {
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
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            #[cfg(any(feature = "phb-http", feature = "phb-file"))]
            Self::PhbUnexpectedFsdTopEntity(suffix) => {
                write!(
                    f,
                    "{suffix} FSD decomposition failed: highest-level entity is not a map"
                )
            }
            #[cfg(feature = "phb-http")]
            Self::PhbHttpInvalidBaseUrl(url, msg) => write!(f, "invalid base URL \"{url}\": {msg}"),
            #[cfg(feature = "phb-http")]
            Self::PhbHttpSuffixJoinFailed(suffix, msg) => {
                write!(f, "{suffix} is failed to be joined to base URL: {msg}")
            }
            #[cfg(feature = "phb-http")]
            Self::PhbHttpSuffixFetchFailed(suffix, msg) => write!(f, "{suffix} fetching failed: {msg}"),
            #[cfg(feature = "phb-http")]
            Self::PhbHttpSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "phb-file")]
            Self::PhbFileSuffixReadFailed(suffix, msg) => write!(f, "{suffix} reading failed: {msg}"),
            #[cfg(feature = "phb-file")]
            Self::PhbFileSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            #[cfg(feature = "phb-file")]
            Self::PhbFileNoClientBuild => write!(f, "unable to find client build field"),
        }
    }
}
