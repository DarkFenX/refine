use crate::err::HExecError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum HBrError {
    #[error("no solar system with ID \"{0}\"")]
    SolNotFound(String),
    #[error("unable to take core solar system")]
    NoCoreSol,
    #[error("fit {0} not found")]
    FitIdCastFailed(String),
    #[error("fleet {0} not found")]
    FleetIdCastFailed(String),
    #[error("item {0} not found")]
    ItemIdCastFailed(String),
    #[error("{0}")]
    ExecFailed(#[from] HExecError),
    #[error("command #{0} failed: {1}")]
    BatchParseFailed(usize, String),
    #[error("command #{0} failed: {1}")]
    BatchExecFailed(usize, #[source] HExecError),
    // Functionality moved to rust interface
    #[error("{0}")]
    SrcCreateFailed(#[from] rs::err::AddSrcError),
    #[error("{0}")]
    SrcGetFailed(#[from] rs::err::GetSrcError),
    #[error("{0}")]
    SrcRemoveFailed(#[from] rs::err::RemoveSrcError),
}
impl HBrError {
    pub(crate) fn from_batch_parse(index: usize, error: impl std::error::Error) -> Self {
        Self::BatchParseFailed(index, error.to_string())
    }
    pub(crate) fn from_batch_exec(index: usize, error: HExecError) -> Self {
        Self::BatchExecFailed(index, error)
    }
    pub(crate) fn get_api_code(&self) -> String {
        match self {
            // Self::SrcAliasNotAvailable(_) => "SRC-001".to_string(),
            // Self::SrcNotFound(_) => "SRC-002".to_string(),
            // Self::NoDefaultSrc => "SRC-003".to_string(),
            Self::SolNotFound(_) => "SOL-001".to_string(),
            Self::NoCoreSol => "SOL-002".to_string(),
            Self::FitIdCastFailed(_) => "FIT-001".to_string(),
            Self::FleetIdCastFailed(_) => "FLT-001".to_string(),
            Self::ItemIdCastFailed(_) => "ITM-001".to_string(),
            Self::SrcCreateFailed(rs::err::AddSrcError::EdhInitFailed(_)) => "EDH-001".to_string(),
            Self::SrcCreateFailed(rs::err::AddSrcError::SrcInitFailed(_)) => "SIN-001".to_string(),
            Self::ExecFailed(e) => e.get_api_code(),
            Self::BatchParseFailed(_, _) => "JSN-002".to_string(),
            Self::BatchExecFailed(_, e) => e.get_api_code(),
            // Functionality moved to rust interface
            // TODO: refine into finer-typed error codes
            Self::SrcCreateFailed(_) => "SRC-001".to_string(),
            Self::SrcGetFailed(_) => "SRC-001".to_string(),
            Self::SrcRemoveFailed(_) => "SRC-001".to_string(),
        }
    }
}
