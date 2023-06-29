use std::{error, fmt};

#[derive(Debug)]
pub(crate) enum HErrorKind {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    SsNotFound(String),
    NoCoreSs,
    FitIdCastFailed(String),
    ItemIdCastFailed(String),
    SettingsInitFailed(String),
    EdhInitFailed(rdhe::ErrorKind, String),
    SrcInitFailed(rc::ErrorKind, String),
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
            HErrorKind::SsNotFound(_) => "SOL-001",
            HErrorKind::NoCoreSs => "SOL-002",
            HErrorKind::FitIdCastFailed(_) => "IDC-001",
            HErrorKind::ItemIdCastFailed(_) => "IDC-002",
            HErrorKind::SettingsInitFailed(_) => "CFG-001",
            HErrorKind::EdhInitFailed(_, _) => "EDH-001",
            HErrorKind::SrcInitFailed(_, _) => "SIN-001",
            HErrorKind::CoreError(k, _) => match k {
                rc::ErrorKind::DhHttpInvalidBaseUrl(_, _) => "COR-001",
                rc::ErrorKind::SrcADataGenFailed(_) => "COR-002",
                rc::ErrorKind::FitNotFound(_) => "COR-003",
                rc::ErrorKind::ItemIdNotFound(_) => "COR-004",
                rc::ErrorKind::SsItemTypeNotFound(_) => "COR-005",
                rc::ErrorKind::FitIdAllocFailed => "COR-006",
                rc::ErrorKind::ItemIdAllocFailed => "COR-007",
                rc::ErrorKind::InvalidSkillLevel(_) => "COR-008",
                rc::ErrorKind::UnexpectedItemType(_, _, _) => "COR-009",
                rc::ErrorKind::ModuleSlotTaken(_, _, _) => "COR-010",
                rc::ErrorKind::AAttrNotFound(_) => "COR-011",
                rc::ErrorKind::AItemNotLoaded(_) => "COR-012",
                rc::ErrorKind::NoAttrBaseValue(_, _) => "COR-013",
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
            HErrorKind::SsNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            HErrorKind::NoCoreSs => write!(f, "unable to take core solar system"),
            HErrorKind::FitIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            HErrorKind::ItemIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            HErrorKind::SettingsInitFailed(reason) => write!(f, "config initialization failed: {reason}"),
            HErrorKind::EdhInitFailed(_, reason) => write!(f, "EVE data handler initialization failed: {reason}"),
            HErrorKind::SrcInitFailed(_, reason) => write!(f, "source initialization failed: {reason}"),
            HErrorKind::CoreError(_, reason) => write!(f, "core library error: {reason}"),
        }
    }
}

pub(crate) type HResult<T> = Result<T, HError>;
