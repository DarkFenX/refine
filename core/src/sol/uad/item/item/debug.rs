use crate::sol::{debug::DebugResult, uad::Uad};

use super::Item;

impl Item {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        match self {
            Self::Autocharge(autocharge) => autocharge.debug_consistency_check(uad),
            Self::Booster(booster) => booster.debug_consistency_check(uad),
            Self::Character(character) => character.debug_consistency_check(uad),
            Self::Charge(charge) => charge.debug_consistency_check(uad),
            Self::Drone(drone) => drone.debug_consistency_check(uad),
            Self::Fighter(fighter) => fighter.debug_consistency_check(uad),
            Self::FwEffect(fw_effect) => fw_effect.debug_consistency_check(uad),
            Self::Implant(implant) => implant.debug_consistency_check(uad),
            Self::Module(module) => module.debug_consistency_check(uad),
            Self::ProjEffect(proj_effect) => proj_effect.debug_consistency_check(uad),
            Self::Rig(rig) => rig.debug_consistency_check(uad),
            Self::Service(service) => service.debug_consistency_check(uad),
            Self::Ship(ship) => ship.debug_consistency_check(uad),
            Self::Skill(skill) => skill.debug_consistency_check(uad),
            Self::Stance(stance) => stance.debug_consistency_check(uad),
            Self::Subsystem(subsystem) => subsystem.debug_consistency_check(uad),
            Self::SwEffect(_) => Ok(()),
        }
    }
}
