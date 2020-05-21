use serde;

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

fn into_opt<T: Into<U>, U>(v: Option<T>) -> Option<U> {
    v.map(Into::into)
}
fn into_vec<T: Into<U>, U>(v: Vec<T>) -> Vec<U> {
    v.into_iter().map(|v| v.into()).collect()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
    pub(super) id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::EveType> for EveType {
    fn into(self) -> dh::EveType {
        dh::EveType::new(self.id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct EveGroup {
    pub(super) id: ReeInt,
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl Into<dh::EveGroup> for EveGroup {
    fn into(self) -> dh::EveGroup {
        dh::EveGroup::new(self.id, self.category_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmAttr {
    pub(super) id: ReeInt,
    pub(super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(super) default: ReeFloat,
}
impl Into<dh::DgmAttr> for DgmAttr {
    fn into(self) -> dh::DgmAttr {
        dh::DgmAttr::new(self.id, self.stackable != 0, self.high_is_good != 0, self.default)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmEffect {
    pub(super) id: ReeInt,
    #[serde(rename = "effectCategory")]
    pub(super) category_id: ReeInt,
    #[serde(rename = "isAssistance")]
    pub(super) is_assistance: ReeInt,
    #[serde(rename = "isOffensive")]
    pub(super) is_offensive: ReeInt,
    #[serde(rename = "isWarpSafe")]
    pub(super) is_warp_safe: ReeInt,
    #[serde(rename = "dischargeAttributeID")]
    pub(super) discharge_attr_id: Option<ReeInt>,
    #[serde(rename = "durationAttributeID")]
    pub(super) duration_attr_id: Option<ReeInt>,
    #[serde(rename = "rangeAttributeID")]
    pub(super) range_attr_id: Option<ReeInt>,
    #[serde(rename = "falloffAttributeID")]
    pub(super) falloff_attr_id: Option<ReeInt>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(super) tracking_attr_id: Option<ReeInt>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(super) usage_chance_attr_id: Option<ReeInt>,
    #[serde(rename = "resistanceAttributeID")]
    pub(super) resist_attr_id: Option<ReeInt>,
    #[serde(rename = "modifierInfo")]
    pub(super) mods: Option<Vec<DgmEffectMod>>,
}
impl Into<dh::DgmEffect> for DgmEffect {
    fn into(self) -> dh::DgmEffect {
        dh::DgmEffect::new(
            self.id,
            self.category_id,
            self.is_assistance != 0,
            self.is_offensive != 0,
            self.is_warp_safe != 0,
            into_opt(self.discharge_attr_id),
            into_opt(self.duration_attr_id),
            into_opt(self.range_attr_id),
            into_opt(self.falloff_attr_id),
            into_opt(self.tracking_attr_id),
            into_opt(self.usage_chance_attr_id),
            into_opt(self.resist_attr_id),
            self.mods.unwrap_or_default().into_iter().map(|v| v.into()).collect(),
        )
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmEffectMod {
    pub(super) func: String,
    pub(super) domain: String,
    #[serde(rename = "modifyingAttributeID")]
    pub(super) src_attr_id: ReeInt,
    pub(super) operation: ReeInt,
    #[serde(rename = "modifiedAttributeID")]
    pub(super) tgt_attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: Option<ReeInt>,
    #[serde(rename = "skillTypeID")]
    pub(super) skill_id: Option<ReeInt>,
}
impl Into<dh::DgmEffectMod> for DgmEffectMod {
    fn into(self) -> dh::DgmEffectMod {
        dh::DgmEffectMod::new(
            self.func,
            self.domain,
            self.src_attr_id,
            self.operation,
            self.tgt_attr_id,
            self.group_id,
            self.skill_id,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Buff {
    pub(super) id: ReeInt,
    #[serde(rename = "aggregateMode")]
    pub(super) aggregate: String,
    #[serde(rename = "operationName")]
    pub(super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(super) item_mods: Vec<BuffItemMod>,
    #[serde(rename = "locationModifiers")]
    pub(super) loc_mods: Vec<BuffLocMod>,
    #[serde(rename = "locationGroupModifiers")]
    pub(super) locgroup_mods: Vec<BuffLocGroupMod>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(super) locsrq_mods: Vec<BuffLocSrqMod>,
}
impl Into<dh::Buff> for Buff {
    fn into(self) -> dh::Buff {
        dh::Buff::new(
            self.id,
            self.aggregate,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffItemMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffItemMod> for BuffItemMod {
    fn into(self) -> dh::BuffItemMod {
        dh::BuffItemMod::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLocMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffLocMod> for BuffLocMod {
    fn into(self) -> dh::BuffLocMod {
        dh::BuffLocMod::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLocGroupMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::BuffLocGroupMod> for BuffLocGroupMod {
    fn into(self) -> dh::BuffLocGroupMod {
        dh::BuffLocGroupMod::new(self.attr_id, self.group_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLocSrqMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(super) skill_id: ReeInt,
}
impl Into<dh::BuffLocSrqMod> for BuffLocSrqMod {
    fn into(self) -> dh::BuffLocSrqMod {
        dh::BuffLocSrqMod::new(self.attr_id, self.skill_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct FighterAbil {
    pub(super) id: ReeInt,
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl Into<dh::FighterAbil> for FighterAbil {
    fn into(self) -> dh::FighterAbil {
        dh::FighterAbil::new(self.id, &self.target_mode, self.disallow_hisec, self.disallow_lowsec)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct TypeFighterAbil {
    pub(super) type_id: ReeInt,
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<AbilExtras>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<AbilExtras>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<AbilExtras>,
}
impl Into<dh::TypeFighterAbil> for TypeFighterAbil {
    fn into(self) -> dh::TypeFighterAbil {
        dh::TypeFighterAbil::new(
            self.type_id,
            into_opt(self.abil0),
            into_opt(self.abil1),
            into_opt(self.abil2),
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
        dh::AbilExtras::new(self.ability_id, self.cooldown, into_opt(self.charges))
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
