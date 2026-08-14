use std::fmt;

use crate::ed::EData;

/// EVE data handler.
///
/// Convenience wrapper to hide boxing necessary to house a handler implementation.
pub struct EveDataHandler(pub Box<dyn EveDataHandlerCore>);
impl EveDataHandler {
    pub fn new<T>(handler: T) -> Self
    where
        T: EveDataHandlerCore + 'static,
    {
        Self(Box::new(handler))
    }
    pub(crate) fn get_impl(&self) -> &dyn EveDataHandlerCore {
        self.0.as_ref()
    }
}
impl<T> From<T> for EveDataHandler
where
    T: EveDataHandlerCore + 'static,
{
    fn from(handler: T) -> Self {
        Self::new(handler)
    }
}
impl fmt::Debug for EveDataHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// EVE data handler interface definition.
pub trait EveDataHandlerCore: fmt::Debug + Send + Sync {
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
    pub fn new<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self(Box::new(error))
    }
}
