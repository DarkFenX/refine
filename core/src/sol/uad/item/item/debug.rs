use crate::sol::{debug::DebugResult, uad::Uad};

use super::Item;

impl Item {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        match self {
            Self::Autocharge(autocharge) => autocharge.consistency_check(uad),
            Self::Booster(booster) => booster.consistency_check(uad),
            Self::Character(character) => character.consistency_check(uad),
            Self::Charge(charge) => charge.consistency_check(uad),
            Self::Drone(drone) => drone.consistency_check(uad),
            Self::Fighter(fighter) => fighter.consistency_check(uad),
            Self::FwEffect(fw_effect) => fw_effect.consistency_check(uad),
            Self::Implant(implant) => implant.consistency_check(uad),
            Self::Module(module) => module.consistency_check(uad),
            Self::ProjEffect(proj_effect) => proj_effect.consistency_check(uad),
            Self::Rig(rig) => rig.consistency_check(uad),
            Self::Service(service) => service.consistency_check(uad),
            Self::Ship(ship) => ship.consistency_check(uad),
            Self::Skill(skill) => skill.consistency_check(uad),
            Self::Stance(stance) => stance.consistency_check(uad),
            Self::Subsystem(subsystem) => subsystem.consistency_check(uad),
            Self::SwEffect(_) => Ok(()),
        }
    }
}
