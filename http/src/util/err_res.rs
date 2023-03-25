use std::{error, fmt, result};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SrcNotFound,
    NoDefaultSrc,
    SrcAliasNotAvailable,
    NoCoreSolSys,
    SettingsInitFailed,
    DhInitFailed(reefast::ErrorKind),
    SrcInitFailed(reefast::ErrorKind),
    CoreError(reefast::ErrorKind),
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) msg: String,
}
impl Error {
    pub(crate) fn new<T: Into<String>>(kind: ErrorKind, msg: T) -> Self {
        Self { kind, msg: msg.into() }
    }
    pub(crate) fn get_code(&self) -> String {
        let code = match self.kind {
            ErrorKind::SrcNotFound => "SRC-001",
            ErrorKind::SrcAliasNotAvailable => "SRC-002",
            _ => "XXX-000",
        };
        code.to_string()
    }
}
impl From<reefast::Error> for Error {
    fn from(err: reefast::Error) -> Self {
        Self::new(ErrorKind::CoreError(err.kind), err.msg)
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;
