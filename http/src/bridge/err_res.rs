use crate::util::HExecError;

#[derive(Debug)]
pub(crate) enum HBrError {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    SolNotFound(String),
    NoCoreSol,
    FitIdCastFailed(String),
    FleetIdCastFailed(String),
    ItemIdCastFailed(String),
    EdhInitFailed(String),
    SrcInitFailed(String),
    ExecFailed(HExecError),
    ExecBatchFailed(usize, HExecError),
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
impl From<HExecError> for HBrError {
    fn from(exec_error: HExecError) -> Self {
        Self::ExecFailed(exec_error)
    }
}
impl std::error::Error for HBrError {}
impl std::fmt::Display for HBrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SrcAliasNotAvailable(alias) => write!(f, "source alias \"{alias}\" is not available"),
            Self::SrcNotFound(alias) => write!(f, "source with alias \"{alias}\" not found"),
            Self::NoDefaultSrc => write!(f, "default source is not defined"),
            Self::SolNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            Self::NoCoreSol => write!(f, "unable to take core solar system"),
            Self::FitIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            Self::FleetIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            Self::ItemIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            Self::EdhInitFailed(reason) => write!(f, "EVE data handler initialization failed: {reason}"),
            Self::SrcInitFailed(reason) => write!(f, "source initialization failed: {reason}"),
            Self::ExecFailed(e) => write!(f, "{e}"),
            Self::ExecBatchFailed(i, e) => write!(f, "command #{} failed: {}", i + 1, e),
        }
    }
}
