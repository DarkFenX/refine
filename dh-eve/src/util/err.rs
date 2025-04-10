#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Unable to parse data due to it being in unexpected format.
    ///
    /// Includes suffix of requested entity.
    #[cfg(any(feature = "phb-http", feature = "phb-file"))]
    #[error("{0} FSD decomposition failed: highest-level entity is not a map")]
    PhbUnexpectedFsdTopEntity(String),
    /// HTTP handler cannot use passed URL as base.
    ///
    /// Includes passed URL and text description of failure.
    #[cfg(feature = "phb-http")]
    #[error("invalid base URL \"{0}\": {1}")]
    PhbHttpInvalidBaseUrl(String, String),
    /// HTTP handler is unable to join base URL and suffix.
    ///
    /// Includes suffix and text description of failure.
    #[cfg(feature = "phb-http")]
    #[error("{0} is failed to be joined to base URL: {1}")]
    PhbHttpSuffixJoinFailed(String, String),
    /// HTTP handler is unable to fetch data.
    ///
    /// Includes suffix and text description of failure.
    #[cfg(feature = "phb-http")]
    #[error("{0} fetching failed: {1}")]
    PhbHttpSuffixFetchFailed(String, String),
    /// HTTP handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    #[cfg(feature = "phb-http")]
    #[error("{0} parsing failed: {1}")]
    PhbHttpSuffixParseFailed(String, String),
    /// File handler is unable to read data.
    ///
    /// Includes suffix and text description of failure.
    #[cfg(feature = "phb-file")]
    #[error("{0} reading failed: {1}")]
    PhbFileSuffixReadFailed(String, String),
    /// File handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    #[cfg(feature = "phb-file")]
    #[error("{0} parsing failed: {1}")]
    PhbFileSuffixParseFailed(String, String),
    /// File handler is unable to find client version in metadata.
    #[cfg(feature = "phb-file")]
    #[error("unable to find client build field")]
    PhbFileNoClientBuild,
}
