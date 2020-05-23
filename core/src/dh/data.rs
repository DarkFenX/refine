use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};

#[derive(Debug)]
pub struct Container<T> {
    pub data: Vec<T>,
    pub failed: u32,
}
impl<T> Container<T> {
    pub fn new(data: Vec<T>, failed: u32) -> Container<T> {
        Container { data, failed }
    }
}

#[derive(Debug)]
pub enum Primitive {
    Null,
    Bool(bool),
    Int(ReeInt),
    Float(ReeFloat),
    String(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct InvType {
    pub id: ReeInt,
    pub group_id: ReeInt,
}
impl InvType {
    pub fn new(id: ReeInt, group_id: ReeInt) -> InvType {
        InvType { id, group_id }
    }
}

#[derive(Debug)]
pub struct InvGroup {
    pub id: ReeInt,
    pub category_id: ReeInt,
}
impl InvGroup {
    pub fn new(id: ReeInt, category_id: ReeInt) -> InvGroup {
        InvGroup { id, category_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct DgmAttr {
    pub id: ReeInt,
    pub stackable: bool,
    pub high_is_good: bool,
    pub default: ReeFloat,
}
impl DgmAttr {
    pub fn new(id: ReeInt, stackable: bool, high_is_good: bool, default: ReeFloat) -> DgmAttr {
        DgmAttr {
            id,
            stackable,
            high_is_good,
            default,
        }
    }
}

#[derive(Debug)]
pub struct DgmTypeAttr {
    pub type_id: ReeInt,
    pub attr_id: ReeInt,
    pub value: ReeFloat,
}
impl DgmTypeAttr {
    pub fn new(type_id: ReeInt, attr_id: ReeInt, value: ReeFloat) -> DgmTypeAttr {
        DgmTypeAttr {
            type_id,
            attr_id,
            value,
        }
    }
}

#[derive(Debug)]
pub struct DgmEffect {
    pub id: ReeInt,
    pub category_id: ReeInt,
    pub is_assistance: bool,
    pub is_offensive: bool,
    pub is_warp_safe: bool,
    pub discharge_attr_id: Option<ReeInt>,
    pub duration_attr_id: Option<ReeInt>,
    pub range_attr_id: Option<ReeInt>,
    pub falloff_attr_id: Option<ReeInt>,
    pub tracking_attr_id: Option<ReeInt>,
    pub usage_chance_attr_id: Option<ReeInt>,
    pub resist_attr_id: Option<ReeInt>,
    pub mods: Vec<DgmEffectMod>,
}
impl DgmEffect {
    pub fn new(
        id: ReeInt,
        category_id: ReeInt,
        is_assistance: bool,
        is_offensive: bool,
        is_warp_safe: bool,
        discharge_attr_id: Option<ReeInt>,
        duration_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        tracking_attr_id: Option<ReeInt>,
        usage_chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
        mods: Vec<DgmEffectMod>,
    ) -> DgmEffect {
        DgmEffect {
            id,
            category_id,
            is_assistance,
            is_offensive,
            is_warp_safe,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            tracking_attr_id,
            usage_chance_attr_id,
            resist_attr_id,
            mods,
        }
    }
}
#[derive(Debug)]
pub struct DgmEffectMod {
    pub func: String,
    pub args: HashMap<String, Primitive>,
}
impl DgmEffectMod {
    pub fn new<T: Into<String>>(func: T, args: HashMap<String, Primitive>) -> DgmEffectMod {
        DgmEffectMod {
            func: func.into(),
            args,
        }
    }
}

#[derive(Debug)]
pub struct DgmTypeEffect {
    pub type_id: ReeInt,
    pub effect_id: ReeInt,
    pub default: bool,
}
impl DgmTypeEffect {
    pub fn new(type_id: ReeInt, effect_id: ReeInt, default: bool) -> DgmTypeEffect {
        DgmTypeEffect {
            type_id,
            effect_id,
            default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct DgmBuff {
    pub id: ReeInt,
    pub aggregate: String,
    pub operation: String,
    pub item_mods: Vec<DgmBuffIM>,
    pub loc_mods: Vec<DgmBuffLM>,
    pub locgroup_mods: Vec<DgmBuffLGM>,
    pub locsrq_mods: Vec<DgmBuffLRSM>,
}
impl DgmBuff {
    pub fn new<T: Into<String>, U: Into<String>>(
        id: ReeInt,
        aggregate: T,
        operation: U,
        item_mods: Vec<DgmBuffIM>,
        loc_mods: Vec<DgmBuffLM>,
        locgroup_mods: Vec<DgmBuffLGM>,
        locsrq_mods: Vec<DgmBuffLRSM>,
    ) -> DgmBuff {
        DgmBuff {
            id,
            aggregate: aggregate.into(),
            operation: operation.into(),
            item_mods,
            loc_mods,
            locgroup_mods,
            locsrq_mods,
        }
    }
}
#[derive(Debug)]
pub struct DgmBuffIM {
    pub attr_id: ReeInt,
}
impl DgmBuffIM {
    pub fn new(attr_id: ReeInt) -> DgmBuffIM {
        DgmBuffIM { attr_id }
    }
}
#[derive(Debug)]
pub struct DgmBuffLM {
    pub attr_id: ReeInt,
}
impl DgmBuffLM {
    pub fn new(attr_id: ReeInt) -> DgmBuffLM {
        DgmBuffLM { attr_id }
    }
}
#[derive(Debug)]
pub struct DgmBuffLGM {
    pub attr_id: ReeInt,
    pub group_id: ReeInt,
}
impl DgmBuffLGM {
    pub fn new(attr_id: ReeInt, group_id: ReeInt) -> DgmBuffLGM {
        DgmBuffLGM { attr_id, group_id }
    }
}
#[derive(Debug)]
pub struct DgmBuffLRSM {
    pub attr_id: ReeInt,
    pub skill_id: ReeInt,
}
impl DgmBuffLRSM {
    pub fn new(attr_id: ReeInt, skill_id: ReeInt) -> DgmBuffLRSM {
        DgmBuffLRSM { attr_id, skill_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct FtrAbil {
    pub id: ReeInt,
    pub target_mode: String,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl FtrAbil {
    pub fn new<T: Into<String>>(id: ReeInt, target_mode: T, disallow_hisec: bool, disallow_lowsec: bool) -> FtrAbil {
        FtrAbil {
            id,
            target_mode: target_mode.into(),
            disallow_hisec,
            disallow_lowsec,
        }
    }
}

#[derive(Debug)]
pub struct FtrTypeAbil {
    pub type_id: ReeInt,
    pub abil0: Option<FtrTypeAbilExtras>,
    pub abil1: Option<FtrTypeAbilExtras>,
    pub abil2: Option<FtrTypeAbilExtras>,
}
impl FtrTypeAbil {
    pub fn new(
        type_id: ReeInt,
        abil0: Option<FtrTypeAbilExtras>,
        abil1: Option<FtrTypeAbilExtras>,
        abil2: Option<FtrTypeAbilExtras>,
    ) -> FtrTypeAbil {
        FtrTypeAbil {
            type_id,
            abil0,
            abil1,
            abil2,
        }
    }
}
#[derive(Debug)]
pub struct FtrTypeAbilExtras {
    pub ability_id: ReeInt,
    pub cooldown: Option<ReeFloat>,
    pub charges: Option<FtrTypeAbilChargeExtras>,
}
impl FtrTypeAbilExtras {
    pub fn new(
        ability_id: ReeInt,
        cooldown: Option<ReeFloat>,
        charges: Option<FtrTypeAbilChargeExtras>,
    ) -> FtrTypeAbilExtras {
        FtrTypeAbilExtras {
            ability_id,
            cooldown,
            charges,
        }
    }
}
#[derive(Debug)]
pub struct FtrTypeAbilChargeExtras {
    pub count: ReeInt,
    pub rearm_time: ReeFloat,
}
impl FtrTypeAbilChargeExtras {
    pub fn new(count: ReeInt, rearm_time: ReeFloat) -> FtrTypeAbilChargeExtras {
        FtrTypeAbilChargeExtras { count, rearm_time }
    }
}
