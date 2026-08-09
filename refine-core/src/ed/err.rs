#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct EveDataHandlerError(#[source] pub Box<dyn std::error::Error + Send + Sync>);
impl EveDataHandlerError {
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}
