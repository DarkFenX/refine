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
    pub fn new(
        id: ReeInt,
        aggregate: String,
        operation: String,
        item_mods: Vec<BuffItemMod>,
        loc_mods: Vec<BuffLocMod>,
        locgroup_mods: Vec<BuffLocGroupMod>,
        locsrq_mods: Vec<BuffLocSrqMod>,
    ) -> Buff {
        Buff {
            id,
            aggregate,
            operation,
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
