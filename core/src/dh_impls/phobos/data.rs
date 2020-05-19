use serde;
use serde_json;

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

pub(super) trait Assemble<T> {
    fn assemble(self, id: ReeInt) -> T;
}

#[derive(Debug)]
pub(super) struct FsdItem {
    pub(super) id: String,
    pub(super) item: serde_json::Value,
}
impl FsdItem {
    pub(super) fn new<T: Into<String>>(id: T, item: serde_json::Value) -> FsdItem {
        FsdItem { id: id.into(), item }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
    pub(super) typeID: ReeInt,
    pub(super) groupID: ReeInt,
}
impl Assemble<dh::EveType> for EveType {
    fn assemble(self, id: ReeInt) -> dh::EveType {
        dh::EveType::new(id, self.groupID)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveGroup {
    pub(super) groupID: ReeInt,
    pub(super) categoryID: ReeInt,
}
impl Assemble<dh::EveGroup> for EveGroup {
    fn assemble(self, id: ReeInt) -> dh::EveGroup {
        dh::EveGroup::new(id, self.categoryID)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct FighterAbil {
    pub(super) targetMode: String,
    pub(super) disallowInHighSec: bool,
    pub(super) disallowInLowSec: bool,
}
impl Assemble<dh::FighterAbil> for FighterAbil {
    fn assemble(self, id: ReeInt) -> dh::FighterAbil {
        dh::FighterAbil::new(id, &self.targetMode, self.disallowInHighSec, self.disallowInLowSec)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct TypeFighterAbil {
    pub(super) abilitySlot0: Option<AbilExtras>,
    pub(super) abilitySlot1: Option<AbilExtras>,
    pub(super) abilitySlot2: Option<AbilExtras>,
}
impl Assemble<dh::TypeFighterAbil> for TypeFighterAbil {
    fn assemble(self, id: ReeInt) -> dh::TypeFighterAbil {
        dh::TypeFighterAbil::new(
            id,
            self.abilitySlot0.map(Into::into),
            self.abilitySlot1.map(Into::into),
            self.abilitySlot2.map(Into::into),
        )
    }
}
#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct AbilExtras {
    pub(super) abilityID: ReeInt,
    pub(super) cooldownSeconds: Option<ReeFloat>,
    pub(super) charges: Option<AbilChargeExtras>,
}
impl Into<dh::AbilExtras> for AbilExtras {
    fn into(self) -> dh::AbilExtras {
        dh::AbilExtras::new(self.abilityID, self.cooldownSeconds, self.charges.map(Into::into))
    }
}
#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct AbilChargeExtras {
    pub(super) chargeCount: ReeInt,
    pub(super) rearmTimeSeconds: ReeFloat,
}
impl Into<dh::AbilChargeExtras> for AbilChargeExtras {
    fn into(self) -> dh::AbilChargeExtras {
        dh::AbilChargeExtras::new(self.chargeCount, self.rearmTimeSeconds)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
