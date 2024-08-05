use crate::util::HExecError;

#[derive(Debug)]
pub(crate) enum HBrErrorKind {
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

#[derive(Debug)]
pub(crate) struct HBrError {
    pub(crate) kind: HBrErrorKind,
}
impl HBrError {
    pub(crate) fn new(kind: HBrErrorKind) -> Self {
        Self { kind }
    }
    pub(crate) fn from_exec_batch(idx: usize, error: HExecError) -> Self {
        Self::new(HBrErrorKind::ExecBatchFailed(idx, error))
    }
    pub(crate) fn get_code(&self) -> String {
        match &self.kind {
            HBrErrorKind::SrcAliasNotAvailable(_) => "SRC-001".to_string(),
            HBrErrorKind::SrcNotFound(_) => "SRC-002".to_string(),
            HBrErrorKind::NoDefaultSrc => "SRC-003".to_string(),
            HBrErrorKind::SolNotFound(_) => "SOL-001".to_string(),
            HBrErrorKind::NoCoreSol => "SOL-002".to_string(),
            HBrErrorKind::FitIdCastFailed(_) => "IDC-001".to_string(),
            HBrErrorKind::FleetIdCastFailed(_) => "IDC-002".to_string(),
            HBrErrorKind::ItemIdCastFailed(_) => "IDC-003".to_string(),
            HBrErrorKind::EdhInitFailed(_) => "EDH-001".to_string(),
            HBrErrorKind::SrcInitFailed(_) => "SIN-001".to_string(),
            HBrErrorKind::ExecFailed(e) => e.get_code(),
            HBrErrorKind::ExecBatchFailed(_, e) => e.get_code(),
        }
    }
}
impl From<HExecError> for HBrError {
    fn from(exec_error: HExecError) -> Self {
        Self::new(HBrErrorKind::ExecFailed(exec_error))
    }
}
impl std::error::Error for HBrError {}
impl std::fmt::Display for HBrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            HBrErrorKind::SrcAliasNotAvailable(alias) => write!(f, "source alias \"{alias}\" is not available"),
            HBrErrorKind::SrcNotFound(alias) => write!(f, "source with alias \"{alias}\" not found"),
            HBrErrorKind::NoDefaultSrc => write!(f, "default source is not defined"),
            HBrErrorKind::SolNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            HBrErrorKind::NoCoreSol => write!(f, "unable to take core solar system"),
            HBrErrorKind::FitIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HBrErrorKind::FleetIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HBrErrorKind::ItemIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HBrErrorKind::EdhInitFailed(reason) => write!(f, "EVE data handler initialization failed: {reason}"),
            HBrErrorKind::SrcInitFailed(reason) => write!(f, "source initialization failed: {reason}"),
            HBrErrorKind::ExecFailed(e) => write!(f, "{e}"),
            HBrErrorKind::ExecBatchFailed(i, e) => write!(f, "command #{} failed: {}", i + 1, e),
        }
    }
}

pub(crate) type HBrResult<T> = Result<T, HBrError>;
