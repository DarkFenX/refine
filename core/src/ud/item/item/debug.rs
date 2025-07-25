use crate::{
    dbg::DebugResult,
    ud::{UData, UItem},
};

impl UItem {
    pub(in crate::ud) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        match self {
            Self::Autocharge(autocharge) => autocharge.consistency_check(u_data),
            Self::Booster(booster) => booster.consistency_check(u_data),
            Self::Character(character) => character.consistency_check(u_data),
            Self::Charge(charge) => charge.consistency_check(u_data),
            Self::Drone(drone) => drone.consistency_check(u_data),
            Self::Fighter(fighter) => fighter.consistency_check(u_data),
            Self::FwEffect(fw_effect) => fw_effect.consistency_check(u_data),
            Self::Implant(implant) => implant.consistency_check(u_data),
            Self::Module(module) => module.consistency_check(u_data),
            Self::ProjEffect(proj_effect) => proj_effect.consistency_check(u_data),
            Self::Rig(rig) => rig.consistency_check(u_data),
            Self::Service(service) => service.consistency_check(u_data),
            Self::Ship(ship) => ship.consistency_check(u_data),
            Self::Skill(skill) => skill.consistency_check(u_data),
            Self::Stance(stance) => stance.consistency_check(u_data),
            Self::Subsystem(subsystem) => subsystem.consistency_check(u_data),
            Self::SwEffect(sw_effect) => sw_effect.consistency_check(u_data),
        }
    }
}
