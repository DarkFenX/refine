use std::{error, fmt, result};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    SsNotFound(String),
    NoCoreSs,
    FitIdCastFailed(String),
    ItemIdCastFailed(String),
    SettingsInitFailed(String),
    EdhInitFailed(rp::ErrorKind, String),
    SrcInitFailed(rc::ErrorKind, String),
    CoreError(rc::ErrorKind, String),
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
    pub(crate) fn get_code(&self) -> String {
        let code = match &self.kind {
            ErrorKind::SrcAliasNotAvailable(_) => "SRC-001",
            ErrorKind::SrcNotFound(_) => "SRC-002",
            ErrorKind::NoDefaultSrc => "SRC-003",
            ErrorKind::SsNotFound(_) => "SOL-001",
            ErrorKind::NoCoreSs => "SOL-002",
            ErrorKind::FitIdCastFailed(_) => "IDC-001",
            ErrorKind::ItemIdCastFailed(_) => "IDC-002",
            ErrorKind::SettingsInitFailed(_) => "CFG-001",
            ErrorKind::EdhInitFailed(_, _) => "EDH-001",
            ErrorKind::SrcInitFailed(_, _) => "SIN-001",
            ErrorKind::CoreError(k, _) => match k {
                rc::ErrorKind::DhHttpInvalidBaseUrl(_, _) => "COR-001",
                rc::ErrorKind::SrcCacheGenFailed(_) => "COR-002",
                rc::ErrorKind::FitNotFound(_) => "COR-003",
                rc::ErrorKind::ItemIdNotFound(_) => "COR-004",
                rc::ErrorKind::ItemTypeNotFound(_) => "COR-005",
                rc::ErrorKind::FitIdAllocFailed => "COR-006",
                rc::ErrorKind::ItemIdAllocFailed => "COR-007",
                rc::ErrorKind::InvalidSkillLevel(_) => "COR-008",
                rc::ErrorKind::UnexpectedItemType(_, _, _) => "COR-009",
                rc::ErrorKind::ModuleSlotTaken(_, _, _) => "COR-010",
                rc::ErrorKind::CachedAttrNotFound(_) => "COR-011",
                rc::ErrorKind::CachedItemNotLoaded(_) => "COR-012",
                rc::ErrorKind::NoAttrBaseValue(_, _) => "COR-013",
            },
        };
        code.to_string()
    }
}
impl From<rc::Error> for Error {
    fn from(err: rc::Error) -> Self {
        let reason = format!("{err}");
        Self::new(ErrorKind::CoreError(err.kind, reason))
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::SrcAliasNotAvailable(alias) => write!(f, "source alias \"{alias}\" is not available"),
            ErrorKind::SrcNotFound(alias) => write!(f, "source with alias \"{alias}\" not found"),
            ErrorKind::NoDefaultSrc => write!(f, "default source is not defined"),
            ErrorKind::SsNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            ErrorKind::NoCoreSs => write!(f, "unable to take core solar system"),
            ErrorKind::FitIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            ErrorKind::ItemIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            ErrorKind::SettingsInitFailed(reason) => write!(f, "config initialization failed: {reason}"),
            ErrorKind::EdhInitFailed(_, reason) => write!(f, "EVE data handler initialization failed: {reason}"),
            ErrorKind::SrcInitFailed(_, reason) => write!(f, "source initialization failed: {reason}"),
            ErrorKind::CoreError(_, reason) => write!(f, "core library error: {reason}"),
        }
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;
