use serde;

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

use super::fsdlite::FsdMerge;

/////////////////////////////////////////////////////////////////////////////
// Inventory data
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl FsdMerge<dh::EveType> for EveType {
    fn fsd_merge(self, id: ReeInt) -> dh::EveType {
        dh::EveType::new(id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct EveGroup {
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl FsdMerge<dh::EveGroup> for EveGroup {
    fn fsd_merge(self, id: ReeInt) -> dh::EveGroup {
        dh::EveGroup::new(id, self.category_id)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Fighter ability data
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct FighterAbil {
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl FsdMerge<dh::FighterAbil> for FighterAbil {
    fn fsd_merge(self, id: ReeInt) -> dh::FighterAbil {
        dh::FighterAbil::new(id, &self.target_mode, self.disallow_hisec, self.disallow_lowsec)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct TypeFighterAbil {
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<AbilExtras>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<AbilExtras>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<AbilExtras>,
}
impl FsdMerge<dh::TypeFighterAbil> for TypeFighterAbil {
    fn fsd_merge(self, id: ReeInt) -> dh::TypeFighterAbil {
        dh::TypeFighterAbil::new(
            id,
            self.abil0.map(Into::into),
            self.abil1.map(Into::into),
            self.abil2.map(Into::into),
        )
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct AbilExtras {
    #[serde(rename = "abilityID")]
    pub(super) ability_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(super) cooldown: Option<ReeFloat>,
    pub(super) charges: Option<AbilChargeExtras>,
}
impl Into<dh::AbilExtras> for AbilExtras {
    fn into(self) -> dh::AbilExtras {
        dh::AbilExtras::new(self.ability_id, self.cooldown, self.charges.map(Into::into))
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct AbilChargeExtras {
    #[serde(rename = "chargeCount")]
    pub(super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(super) rearm_time: ReeFloat,
}
impl Into<dh::AbilChargeExtras> for AbilChargeExtras {
    fn into(self) -> dh::AbilChargeExtras {
        dh::AbilChargeExtras::new(self.count, self.rearm_time)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Misc data
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
