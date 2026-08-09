#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EveDataHandlerError(pub Box<dyn std::error::Error + Send + Sync>);
impl EveDataHandlerError {
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}
