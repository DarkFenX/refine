use crate::sol::{SolDebugResult, SolView};

use super::SolItem;

impl SolItem {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        match self {
            Self::Autocharge(autocharge) => autocharge.debug_consistency_check(sol_view),
            Self::Booster(booster) => booster.debug_consistency_check(sol_view),
            Self::Character(character) => character.debug_consistency_check(sol_view),
            Self::Charge(charge) => charge.debug_consistency_check(sol_view),
            Self::Drone(drone) => drone.debug_consistency_check(sol_view),
            Self::Fighter(fighter) => fighter.debug_consistency_check(sol_view),
            Self::FwEffect(fw_effect) => fw_effect.debug_consistency_check(sol_view),
            Self::Implant(implant) => implant.debug_consistency_check(sol_view),
            Self::Module(module) => module.debug_consistency_check(sol_view),
            Self::ProjEffect(proj_effect) => proj_effect.debug_consistency_check(sol_view),
            Self::Rig(rig) => rig.debug_consistency_check(sol_view),
            Self::Ship(ship) => ship.debug_consistency_check(sol_view),
            Self::Skill(skill) => skill.debug_consistency_check(sol_view),
            Self::Stance(stance) => stance.debug_consistency_check(sol_view),
            Self::Subsystem(subsystem) => subsystem.debug_consistency_check(sol_view),
            Self::SwEffect(_) => Ok(()),
        }
    }
}
