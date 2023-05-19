use std::{error, fmt, result};

use crate::{
    consts::ModRack,
    defs::{ReeId, ReeIdx, ReeInt},
};

/// Defines error types which are returned by the library.
#[derive(Debug)]
pub enum ErrorKind {
    DhHttpInvalidBaseUrl(String, String),
    SrcCacheGenFailed(String),
    FitNotFound(ReeId),
    ItemIdNotFound(ReeId),
    ItemTypeNotFound(&'static str),
    FitIdAllocFailed,
    ItemIdAllocFailed,
    InvalidSkillLevel(ReeInt),
    UnexpectedItemType(ReeId, &'static str, &'static str),
    ModuleSlotTaken(ModRack, ReeIdx, ReeId),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}
impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::DhHttpInvalidBaseUrl(url, msg) => write!(f, "invalid \"{url}\": {msg}"),
            ErrorKind::SrcCacheGenFailed(reason) => write!(f, "cache generation failed: {reason}"),
            ErrorKind::FitNotFound(fit_id) => write!(f, "fit {fit_id} not found"),
            ErrorKind::ItemIdNotFound(item_id) => write!(f, "item {item_id} not found"),
            ErrorKind::ItemTypeNotFound(item_type) => write!(f, "{item_type} not found"),
            ErrorKind::FitIdAllocFailed => write!(f, "fit ID allocation failed"),
            ErrorKind::ItemIdAllocFailed => write!(f, "item ID allocation failed"),
            ErrorKind::InvalidSkillLevel(level) => write!(f, "skill level {level} is out of allowed range [0, 5]"),
            ErrorKind::UnexpectedItemType(item_id, actual, expected) => {
                write!(f, "item {item_id} was requested as {expected}. but is {actual}")
            }
            ErrorKind::ModuleSlotTaken(rack, position, item_id) => {
                write!(f, "{rack} slot {position} is occupied by item {item_id}")
            }
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
