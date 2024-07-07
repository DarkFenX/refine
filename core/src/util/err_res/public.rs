use std::{error, fmt, result};

use crate::{
    ad,
    defs::{EAttrId, EItemId, Idx, SkillLevel, SolFitId, SolFleetId, SolItemId},
    sol::SolModRack,
    util::Named,
};

/// Defines error types which are returned by the library.
#[derive(Debug)]
pub enum ErrorKind {
    DhHttpInvalidBaseUrl(String, String),
    SrcADataGenFailed(String),
    ItemIdNotFound(SolItemId),
    FitNotFound(SolFitId),
    FleetNotFound(SolFleetId),
    SolItemKindNotFound(&'static str),
    ItemIdAllocFailed,
    FitIdAllocFailed,
    FleetIdAllocFailed,
    InvalidSkillLevel(SkillLevel),
    UnexpectedItemKind(SolItemId, &'static str, &'static str),
    ModuleSlotTaken(SolModRack, Idx, SolItemId),
    AAttrNotFound(EAttrId),
    AItemNotLoaded(EItemId),
    CustomModCalc,
    ItemNotProjectable(SolItemId),
    ProjecteeNotFound(SolItemId, SolItemId),
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
            ErrorKind::ItemIdNotFound(item_id) => write!(f, "item {item_id} not found"),
            ErrorKind::FitNotFound(fit_id) => write!(f, "fit {fit_id} not found"),
            ErrorKind::FleetNotFound(fleet_id) => write!(f, "fleet {fleet_id} not found"),
            ErrorKind::SolItemKindNotFound(item_kind) => write!(f, "{item_kind} not found"),
            ErrorKind::ItemIdAllocFailed => write!(f, "item ID allocation failed"),
            ErrorKind::FitIdAllocFailed => write!(f, "fit ID allocation failed"),
            ErrorKind::FleetIdAllocFailed => write!(f, "fit ID allocation failed"),
            ErrorKind::InvalidSkillLevel(level) => write!(f, "skill level {level} is out of allowed range [0, 5]"),
            ErrorKind::UnexpectedItemKind(item_id, actual, expected) => {
                write!(f, "item {item_id} was requested as {expected}. but is {actual}")
            }
            ErrorKind::ModuleSlotTaken(rack, position, item_id) => {
                write!(f, "{rack} slot {position} is occupied by item {item_id}")
            }
            ErrorKind::AAttrNotFound(attr_id) => write!(f, "{}(id={}) not found", ad::AAttr::get_name(), attr_id),
            ErrorKind::AItemNotLoaded(type_id) => write!(f, "{}(id={}) not found", ad::AItem::get_name(), type_id),
            ErrorKind::CustomModCalc => write!(f, "failed to calculate custom modifier"),
            ErrorKind::ItemNotProjectable(item_id) => write!(f, "item {item_id} cannot be projected onto"),
            ErrorKind::ProjecteeNotFound(projector_item_id, projectee_item_id) => {
                write!(
                    f,
                    "item {projector_item_id} doesn't have item {projectee_item_id} as projection"
                )
            }
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
