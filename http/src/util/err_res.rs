use std::{error, fmt, result};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SrcAliasNotAvailable(String),
    SrcNotFound(String),
    NoDefaultSrc,
    NoCoreSolSys,
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
        let code = match self.kind {
            ErrorKind::SrcNotFound(_) => "SRC-001",
            ErrorKind::SrcAliasNotAvailable(_) => "SRC-002",
            _ => "XXX-000",
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
            ErrorKind::NoCoreSolSys => write!(f, "unable to take core solar system"),
            ErrorKind::SettingsInitFailed(reason) => write!(f, "config initialization failed: {reason}"),
            ErrorKind::DhInitFailed(_, reason) => write!(f, "data handler initialization failed: {reason}"),
            ErrorKind::SrcInitFailed(_, reason) => write!(f, "source initialization failed: {reason}"),
            ErrorKind::CoreError(_, reason) => write!(f, "core library error: {reason}"),
        }
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;
