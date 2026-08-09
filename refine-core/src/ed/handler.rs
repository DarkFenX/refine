use crate::ed::EData;

/// EVE data handler interface definition.
pub trait EveDataHandler: std::fmt::Debug + Send + Sync {
    /// Get main EVE data.
    ///
    /// This method should return an error only when it is impossible to fetch the data altogether.
    /// In case of a less impactful error (such as inability to deserialize one specific item within
    /// a big array of data), the error should be recorded as a meaningful warning message and
    /// stored in [`EDataCont::warnings`](crate::ed::EDataCont::warnings) for appropriate entity.
    fn get_data(&self) -> Result<EData, EveDataHandlerError>;
    /// Get version of the data.
    fn get_data_version(&self) -> Result<String, EveDataHandlerError>;
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EveDataHandlerError(pub Box<dyn std::error::Error + Send + Sync>);
impl EveDataHandlerError {
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}
