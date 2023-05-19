use std::{error, fmt, result};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    SolSysNotFound(String),
    NoCoreSolSys,
    FitIdCastFailed(String),
    ItemIdCastFailed(String),
    SettingsInitFailed(String),
    DhInitFailed(reefast::ErrorKind, String),
    SrcInitFailed(reefast::ErrorKind, String),
    CoreError(reefast::ErrorKind, String),
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
            ErrorKind::SolSysNotFound(_) => "SOL-001",
            ErrorKind::NoCoreSolSys => "SOL-002",
            ErrorKind::FitIdCastFailed(_) => "IDC-001",
            ErrorKind::ItemIdCastFailed(_) => "IDC-002",
            ErrorKind::SettingsInitFailed(_) => "CFG-001",
            ErrorKind::DhInitFailed(_, _) => "DHR-001",
            ErrorKind::SrcInitFailed(_, _) => "SIN-001",
            ErrorKind::CoreError(k, _) => match k {
                reefast::ErrorKind::DhHttpInvalidBaseUrl => "COR-001",
                reefast::ErrorKind::SrcCacheGenFailed => "COR-002",
                reefast::ErrorKind::SrcNotFound => "COR-003",
                reefast::ErrorKind::AlreadyHasParent => "COR-004",
                reefast::ErrorKind::FitNotFound => "COR-005",
                reefast::ErrorKind::ItemIdNotFound => "COR-006",
                reefast::ErrorKind::IdAllocFailed => "COR-007",
                reefast::ErrorKind::InvalidSkillLevel => "COR-008",
                reefast::ErrorKind::UnexpectedItemType => "COR-009",
                reefast::ErrorKind::ModuleSlotTaken => "COR-010",
            },
        };
        code.to_string()
    }
}
impl From<reefast::Error> for Error {
    fn from(err: reefast::Error) -> Self {
        Self::new(ErrorKind::CoreError(err.kind, err.msg))
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::SrcAliasNotAvailable(alias) => write!(f, "source alias \"{alias}\" is not available"),
            ErrorKind::SrcNotFound(alias) => write!(f, "source with alias \"{alias}\" not found"),
            ErrorKind::NoDefaultSrc => write!(f, "default source is not defined"),
            ErrorKind::SolSysNotFound(id) => write!(f, "no solar system with ID \"{id}\""),
            ErrorKind::NoCoreSolSys => write!(f, "unable to take core solar system"),
            ErrorKind::FitIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            ErrorKind::ItemIdCastFailed(s) => write!(f, "unable to take cast string {s} to id"),
            ErrorKind::SettingsInitFailed(reason) => write!(f, "config initialization failed: {reason}"),
            ErrorKind::DhInitFailed(_, reason) => write!(f, "data handler initialization failed: {reason}"),
            ErrorKind::SrcInitFailed(_, reason) => write!(f, "source initialization failed: {reason}"),
            ErrorKind::CoreError(_, reason) => write!(f, "core library error: {reason}"),
        }
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;
