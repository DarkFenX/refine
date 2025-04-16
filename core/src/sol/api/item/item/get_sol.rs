use crate::sol::{
    SolarSystem,
    api::{Item, ItemMut},
};

impl<'a> Item<'a> {
    pub(in crate::sol) fn get_sol(&self) -> &SolarSystem {
        match self {
            Item::Autocharge(autocharge) => autocharge.sol,
            Item::Booster(booster) => booster.sol,
            Item::Character(character) => character.sol,
            Item::Charge(charge) => charge.sol,
            Item::Drone(drone) => drone.sol,
            Item::Fighter(fighter) => fighter.sol,
            Item::FwEffect(fw_effect) => fw_effect.sol,
            Item::Implant(implant) => implant.sol,
            Item::Module(module) => module.sol,
            Item::ProjEffect(proj_effect) => proj_effect.sol,
            Item::Rig(rig) => rig.sol,
            Item::Service(service) => service.sol,
            Item::Ship(ship) => ship.sol,
            Item::Skill(skill) => skill.sol,
            Item::Stance(stance) => stance.sol,
            Item::Subsystem(subsystem) => subsystem.sol,
            Item::SwEffect(sw_effect) => sw_effect.sol,
        }
    }
}

impl<'a> ItemMut<'a> {
    pub(in crate::sol) fn get_sol(&self) -> &SolarSystem {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.sol,
            ItemMut::Booster(booster) => booster.sol,
            ItemMut::Character(character) => character.sol,
            ItemMut::Charge(charge) => charge.sol,
            ItemMut::Drone(drone) => drone.sol,
            ItemMut::Fighter(fighter) => fighter.sol,
            ItemMut::FwEffect(fw_effect) => fw_effect.sol,
            ItemMut::Implant(implant) => implant.sol,
            ItemMut::Module(module) => module.sol,
            ItemMut::ProjEffect(proj_effect) => proj_effect.sol,
            ItemMut::Rig(rig) => rig.sol,
            ItemMut::Service(service) => service.sol,
            ItemMut::Ship(ship) => ship.sol,
            ItemMut::Skill(skill) => skill.sol,
            ItemMut::Stance(stance) => stance.sol,
            ItemMut::Subsystem(subsystem) => subsystem.sol,
            ItemMut::SwEffect(sw_effect) => sw_effect.sol,
        }
    }
    pub(in crate::sol) fn get_sol_mut(&mut self) -> &mut SolarSystem {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.sol,
            ItemMut::Booster(booster) => booster.sol,
            ItemMut::Character(character) => character.sol,
            ItemMut::Charge(charge) => charge.sol,
            ItemMut::Drone(drone) => drone.sol,
            ItemMut::Fighter(fighter) => fighter.sol,
            ItemMut::FwEffect(fw_effect) => fw_effect.sol,
            ItemMut::Implant(implant) => implant.sol,
            ItemMut::Module(module) => module.sol,
            ItemMut::ProjEffect(proj_effect) => proj_effect.sol,
            ItemMut::Rig(rig) => rig.sol,
            ItemMut::Service(service) => service.sol,
            ItemMut::Ship(ship) => ship.sol,
            ItemMut::Skill(skill) => skill.sol,
            ItemMut::Stance(stance) => stance.sol,
            ItemMut::Subsystem(subsystem) => subsystem.sol,
            ItemMut::SwEffect(sw_effect) => sw_effect.sol,
        }
    }
}
