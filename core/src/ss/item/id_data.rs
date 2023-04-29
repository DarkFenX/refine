use crate::{
    ss::item::{
        Booster, Character, Charge, Drone, Fighter, Implant, Module, Rig, Ship, Skill, Stance, Subsystem, SwEffect,
    },
    ReeId,
};

pub struct IdData {
    pub item_id: ReeId,
    pub charge_id: Option<ReeId>,
    pub autocharge_ids: Option<Vec<ReeId>>,
}
impl IdData {
    pub(in crate::ss) fn new(item_id: ReeId) -> Self {
        Self {
            item_id,
            charge_id: None,
            autocharge_ids: None,
        }
    }
    pub(in crate::ss) fn new_with_charges(
        item_id: ReeId,
        charge_id: Option<ReeId>,
        autocharge_ids: Option<Vec<ReeId>>,
    ) -> Self {
        Self {
            item_id,
            charge_id,
            autocharge_ids,
        }
    }
}

impl From<&Booster> for IdData {
    fn from(value: &Booster) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Character> for IdData {
    fn from(value: &Character) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Charge> for IdData {
    fn from(value: &Charge) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Drone> for IdData {
    fn from(value: &Drone) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Fighter> for IdData {
    fn from(value: &Fighter) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Implant> for IdData {
    fn from(value: &Implant) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Module> for IdData {
    fn from(value: &Module) -> Self {
        IdData::new_with_charges(value.item_id, value.charge, None)
    }
}
impl From<&Rig> for IdData {
    fn from(value: &Rig) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Ship> for IdData {
    fn from(value: &Ship) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Skill> for IdData {
    fn from(value: &Skill) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Stance> for IdData {
    fn from(value: &Stance) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&Subsystem> for IdData {
    fn from(value: &Subsystem) -> Self {
        IdData::new(value.item_id)
    }
}
impl From<&SwEffect> for IdData {
    fn from(value: &SwEffect) -> Self {
        IdData::new(value.item_id)
    }
}
