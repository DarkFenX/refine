use crate::ad::err::ADataGeneratorError;

#[derive(Debug, thiserror::Error)]
pub enum SrcInitError {
    #[error(transparent)]
    ADataGeneration(#[from] ADataGeneratorError),
}
