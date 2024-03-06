use std::{error, fmt, result};

use crate::{
    ad,
    defs::{EAttrId, EEffectId, EItemId, Idx, SkillLevel, SsFitId, SsItemId},
    ss::ModRack,
    util::Named,
};

/// Defines error types which are returned by the library.
#[derive(Debug)]
pub enum ErrorKind {
    DhHttpInvalidBaseUrl(String, String),
    SrcADataGenFailed(String),
    FitNotFound(SsFitId),
    ItemIdNotFound(SsItemId),
    SsItemTypeNotFound(&'static str),
    FitIdAllocFailed,
    ItemIdAllocFailed,
    InvalidSkillLevel(SkillLevel),
    UnexpectedItemType(SsItemId, &'static str, &'static str),
    ModuleSlotTaken(ModRack, Idx, SsItemId),
    AAttrNotFound(EAttrId),
    AItemNotLoaded(EItemId),
    NoAttrBaseValue(EAttrId, EItemId),
    CustomModCalc,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::DhHttpInvalidBaseUrl(url, msg) => write!(f, "invalid \"{url}\": {msg}"),
            ErrorKind::SrcADataGenFailed(reason) => write!(f, "adapted data generation failed: {reason}"),
            ErrorKind::FitNotFound(fit_id) => write!(f, "fit {fit_id} not found"),
            ErrorKind::ItemIdNotFound(item_id) => write!(f, "item {item_id} not found"),
            ErrorKind::SsItemTypeNotFound(item_type) => write!(f, "{item_type} not found"),
            ErrorKind::FitIdAllocFailed => write!(f, "fit ID allocation failed"),
            ErrorKind::ItemIdAllocFailed => write!(f, "item ID allocation failed"),
            ErrorKind::InvalidSkillLevel(level) => write!(f, "skill level {level} is out of allowed range [0, 5]"),
            ErrorKind::UnexpectedItemType(item_id, actual, expected) => {
                write!(f, "item {item_id} was requested as {expected}. but is {actual}")
            }
            ErrorKind::ModuleSlotTaken(rack, position, item_id) => {
                write!(f, "{rack} slot {position} is occupied by item {item_id}")
            }
            ErrorKind::AAttrNotFound(attr_id) => write!(f, "{}(id={}) not found", ad::AAttr::get_name(), attr_id),
            ErrorKind::AItemNotLoaded(type_id) => write!(f, "{}(id={}) not found", ad::AItem::get_name(), type_id),
            ErrorKind::NoAttrBaseValue(attr_id, type_id) => write!(
                f,
                "{} {} has no base value for {} {}",
                ad::AAttr::get_name(),
                attr_id,
                ad::AItem::get_name(),
                type_id
            ),
            ErrorKind::CustomModCalc => write!(f, "failed to calculate custom modifier"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
