use std::{error, fmt};

#[derive(Debug)]
pub(crate) enum HErrorKind {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    SolNotFound(String),
    NoCoreSol,
    FitIdCastFailed(String),
    FleetIdCastFailed(String),
    ItemIdCastFailed(String),
    SettingsInitFailed(String),
    EdhInitFailed(String),
    SrcInitFailed(String),
    CoreError(rc::ErrorKind, String),
}

#[derive(Debug)]
pub(crate) struct HError {
    pub(crate) kind: HErrorKind,
}
impl HError {
    pub(crate) fn new(kind: HErrorKind) -> Self {
        Self { kind }
    }
    pub(crate) fn get_code(&self) -> String {
        let code = match &self.kind {
            HErrorKind::SrcAliasNotAvailable(_) => "SRC-001",
            HErrorKind::SrcNotFound(_) => "SRC-002",
            HErrorKind::NoDefaultSrc => "SRC-003",
            HErrorKind::SolNotFound(_) => "SOL-001",
            HErrorKind::NoCoreSol => "SOL-002",
            HErrorKind::FitIdCastFailed(_) => "IDC-001",
            HErrorKind::FleetIdCastFailed(_) => "IDC-002",
            HErrorKind::ItemIdCastFailed(_) => "IDC-003",
            HErrorKind::SettingsInitFailed(_) => "CFG-001",
            HErrorKind::EdhInitFailed(_) => "EDH-001",
            HErrorKind::SrcInitFailed(_) => "SIN-001",
            HErrorKind::CoreError(k, _) => match k {
                rc::ErrorKind::DhHttpInvalidBaseUrl(_, _) => "COR-001",
                rc::ErrorKind::SrcADataGenFailed(_) => "COR-002",
                rc::ErrorKind::FitNotFound(_) => "COR-003",
                rc::ErrorKind::ItemIdNotFound(_) => "COR-004",
                rc::ErrorKind::SolItemKindNotFound(_) => "COR-005",
                rc::ErrorKind::FitIdAllocFailed => "COR-006",
                rc::ErrorKind::ItemIdAllocFailed => "COR-007",
                rc::ErrorKind::InvalidSkillLevel(_) => "COR-008",
                rc::ErrorKind::UnexpectedItemKind(_, _, _) => "COR-009",
                rc::ErrorKind::ModuleSlotTaken(_, _, _) => "COR-010",
                rc::ErrorKind::AAttrNotFound(_) => "COR-011",
                rc::ErrorKind::AItemNotLoaded(_) => "COR-012",
                rc::ErrorKind::CustomModCalc => "COR-014",
                rc::ErrorKind::ItemNotProjectable(_) => "COR-015",
                rc::ErrorKind::FleetIdAllocFailed => "COR-016",
                rc::ErrorKind::FleetNotFound(_) => "COR-017",
                rc::ErrorKind::ProjecteeNotFound(_, _) => "COR-018",
            },
        };
        code.to_string()
    }
}
impl From<rc::Error> for HError {
    fn from(core_err: rc::Error) -> Self {
        let reason = format!("{core_err}");
        Self::new(HErrorKind::CoreError(core_err.kind, reason))
    }
}
impl error::Error for HError {}
impl fmt::Display for HError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            HErrorKind::SrcAliasNotAvailable(alias) => write!(f, "source alias \"{alias}\" is not available"),
            HErrorKind::SrcNotFound(alias) => write!(f, "source with alias \"{alias}\" not found"),
            HErrorKind::NoDefaultSrc => write!(f, "default source is not defined"),
            HErrorKind::SolNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            HErrorKind::NoCoreSol => write!(f, "unable to take core solar system"),
            HErrorKind::FitIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HErrorKind::FleetIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HErrorKind::ItemIdCastFailed(s) => write!(f, "unable to cast string \"{s}\" to id"),
            HErrorKind::SettingsInitFailed(reason) => write!(f, "config initialization failed: {reason}"),
            HErrorKind::EdhInitFailed(reason) => write!(f, "EVE data handler initialization failed: {reason}"),
            HErrorKind::SrcInitFailed(reason) => write!(f, "source initialization failed: {reason}"),
            HErrorKind::CoreError(_, reason) => write!(f, "core library error: {reason}"),
        }
    }
}

pub(crate) type HResult<T> = Result<T, HError>;
