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
    pub id: ReeInt,
    pub cooldown: Option<ReeFloat>,
    pub charges: Option<AbilChargeExtras>,
}
impl AbilExtras {
    pub fn new(id: ReeInt, cooldown: Option<ReeFloat>, charges: Option<AbilChargeExtras>) -> AbilExtras {
        AbilExtras { id, cooldown, charges }
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
