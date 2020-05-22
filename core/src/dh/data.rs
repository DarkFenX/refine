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
pub struct EveType {
    pub id: ReeInt,
    pub group_id: ReeInt,
}
impl EveType {
    pub fn new(id: ReeInt, group_id: ReeInt) -> EveType {
        EveType { id, group_id }
    }
}

#[derive(Debug)]
pub struct EveGroup {
    pub id: ReeInt,
    pub category_id: ReeInt,
}
impl EveGroup {
    pub fn new(id: ReeInt, category_id: ReeInt) -> EveGroup {
        EveGroup { id, category_id }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Buff {
    pub id: ReeInt,
    pub aggregate: String,
    pub operation: String,
    pub item_mods: Vec<BuffItemMod>,
    pub loc_mods: Vec<BuffLocMod>,
    pub locgroup_mods: Vec<BuffLocGroupMod>,
    pub locsrq_mods: Vec<BuffLocSrqMod>,
}
impl Buff {
    pub fn new<T: Into<String>, U: Into<String>>(
        id: ReeInt,
        aggregate: T,
        operation: U,
        item_mods: Vec<BuffItemMod>,
        loc_mods: Vec<BuffLocMod>,
        locgroup_mods: Vec<BuffLocGroupMod>,
        locsrq_mods: Vec<BuffLocSrqMod>,
    ) -> Buff {
        Buff {
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
pub struct BuffItemMod {
    pub attr_id: ReeInt,
}
impl BuffItemMod {
    pub fn new(attr_id: ReeInt) -> BuffItemMod {
        BuffItemMod { attr_id }
    }
}
#[derive(Debug)]
pub struct BuffLocMod {
    pub attr_id: ReeInt,
}
impl BuffLocMod {
    pub fn new(attr_id: ReeInt) -> BuffLocMod {
        BuffLocMod { attr_id }
    }
}
#[derive(Debug)]
pub struct BuffLocGroupMod {
    pub attr_id: ReeInt,
    pub group_id: ReeInt,
}
impl BuffLocGroupMod {
    pub fn new(attr_id: ReeInt, group_id: ReeInt) -> BuffLocGroupMod {
        BuffLocGroupMod { attr_id, group_id }
    }
}
#[derive(Debug)]
pub struct BuffLocSrqMod {
    pub attr_id: ReeInt,
    pub skill_id: ReeInt,
}
impl BuffLocSrqMod {
    pub fn new(attr_id: ReeInt, skill_id: ReeInt) -> BuffLocSrqMod {
        BuffLocSrqMod { attr_id, skill_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct FighterAbil {
    pub id: ReeInt,
    pub target_mode: String,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl FighterAbil {
    pub fn new<T: Into<String>>(
        id: ReeInt,
        target_mode: T,
        disallow_hisec: bool,
        disallow_lowsec: bool,
    ) -> FighterAbil {
        FighterAbil {
            id,
            target_mode: target_mode.into(),
            disallow_hisec,
            disallow_lowsec,
        }
    }
}

#[derive(Debug)]
pub struct TypeFighterAbil {
    pub type_id: ReeInt,
    pub abil0: Option<AbilExtras>,
    pub abil1: Option<AbilExtras>,
    pub abil2: Option<AbilExtras>,
}
impl TypeFighterAbil {
    pub fn new(
        type_id: ReeInt,
        abil0: Option<AbilExtras>,
        abil1: Option<AbilExtras>,
        abil2: Option<AbilExtras>,
    ) -> TypeFighterAbil {
        TypeFighterAbil {
            type_id,
            abil0,
            abil1,
            abil2,
        }
    }
}
#[derive(Debug)]
pub struct AbilExtras {
    pub ability_id: ReeInt,
    pub cooldown: Option<ReeFloat>,
    pub charges: Option<AbilChargeExtras>,
}
impl AbilExtras {
    pub fn new(ability_id: ReeInt, cooldown: Option<ReeFloat>, charges: Option<AbilChargeExtras>) -> AbilExtras {
        AbilExtras {
            ability_id,
            cooldown,
            charges,
        }
    }
}
#[derive(Debug)]
pub struct AbilChargeExtras {
    pub count: ReeInt,
    pub rearm_time: ReeFloat,
}
impl AbilChargeExtras {
    pub fn new(count: ReeInt, rearm_time: ReeFloat) -> AbilChargeExtras {
        AbilChargeExtras { count, rearm_time }
    }
}
