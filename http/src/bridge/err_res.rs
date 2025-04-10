use crate::util::HExecError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum HBrError {
    #[error("source alias \"{0}\" is not available")]
    SrcAliasNotAvailable(String),
    #[error("source with alias \"{0}\" not found")]
    SrcNotFound(String),
    #[error("default source is not defined")]
    NoDefaultSrc,
    #[error("no solar system with ID \"{0}\"")]
    SolNotFound(String),
    #[error("unable to take core solar system")]
    NoCoreSol,
    #[error("unable to cast string \"{0}\" to id")]
    FitIdCastFailed(String),
    #[error("unable to cast string \"{0}\" to id")]
    FleetIdCastFailed(String),
    #[error("unable to cast string \"{0}\" to id")]
    ItemIdCastFailed(String),
    #[error("EVE data handler initialization failed: {0}")]
    EdhInitFailed(String),
    #[error("source initialization failed: {0}")]
    SrcInitFailed(String),
    #[error("{0}")]
    ExecFailed(#[from] HExecError),
    #[error("command #{i} failed: {1}", i = .0 + 1)]
    ExecBatchFailed(usize, #[source] HExecError),
}
impl HBrError {
    pub(crate) fn from_exec_batch(idx: usize, error: HExecError) -> Self {
        Self::ExecBatchFailed(idx, error)
    }
    pub(crate) fn get_code(&self) -> String {
        match self {
            Self::SrcAliasNotAvailable(_) => "SRC-001".to_string(),
            Self::SrcNotFound(_) => "SRC-002".to_string(),
            Self::NoDefaultSrc => "SRC-003".to_string(),
            Self::SolNotFound(_) => "SOL-001".to_string(),
            Self::NoCoreSol => "SOL-002".to_string(),
            Self::FitIdCastFailed(_) => "IDC-001".to_string(),
            Self::FleetIdCastFailed(_) => "IDC-002".to_string(),
            Self::ItemIdCastFailed(_) => "IDC-003".to_string(),
            Self::EdhInitFailed(_) => "EDH-001".to_string(),
            Self::SrcInitFailed(_) => "SIN-001".to_string(),
            Self::ExecFailed(e) => e.get_code(),
            Self::ExecBatchFailed(_, e) => e.get_code(),
        }
    }
}
