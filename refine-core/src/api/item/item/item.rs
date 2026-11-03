use crate::{
    Autocharge, Booster, Character, Charge, Drone, Fighter, FwEffect, Implant, ItemCommon, Module, ProjEffect, Rig,
    Service, Ship, Skill, SolarSystem, Stance, Subsystem, SwEffect, api::ItemSealed, ud::UItemId,
};

pub enum Item<'s> {
    Autocharge(Autocharge<'s>),
    Booster(Booster<'s>),
    Character(Character<'s>),
    Charge(Charge<'s>),
    Drone(Drone<'s>),
    Fighter(Fighter<'s>),
    FwEffect(FwEffect<'s>),
    Implant(Implant<'s>),
    Module(Module<'s>),
    ProjEffect(ProjEffect<'s>),
    Rig(Rig<'s>),
    Service(Service<'s>),
    Ship(Ship<'s>),
    Skill(Skill<'s>),
    Stance(Stance<'s>),
    Subsystem(Subsystem<'s>),
    SwEffect(SwEffect<'s>),
}
impl<'s> Item<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        sol.internal_get_item(uid)
    }
}
impl<'s> ItemSealed for Item<'s> {
    fn get_uid(&self) -> UItemId {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_uid(),
            Item::Booster(booster) => booster.get_uid(),
            Item::Character(character) => character.get_uid(),
            Item::Charge(charge) => charge.get_uid(),
            Item::Drone(drone) => drone.get_uid(),
            Item::Fighter(fighter) => fighter.get_uid(),
            Item::FwEffect(fw_effect) => fw_effect.get_uid(),
            Item::Implant(implant) => implant.get_uid(),
            Item::Module(module) => module.get_uid(),
            Item::ProjEffect(proj_effect) => proj_effect.get_uid(),
            Item::Rig(rig) => rig.get_uid(),
            Item::Service(service) => service.get_uid(),
            Item::Ship(ship) => ship.get_uid(),
            Item::Skill(skill) => skill.get_uid(),
            Item::Stance(stance) => stance.get_uid(),
            Item::Subsystem(subsystem) => subsystem.get_uid(),
            Item::SwEffect(sw_effect) => sw_effect.get_uid(),
        }
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_sol_ref(),
            Item::Booster(booster) => booster.get_sol_ref(),
            Item::Character(character) => character.get_sol_ref(),
            Item::Charge(charge) => charge.get_sol_ref(),
            Item::Drone(drone) => drone.get_sol_ref(),
            Item::Fighter(fighter) => fighter.get_sol_ref(),
            Item::FwEffect(fw_effect) => fw_effect.get_sol_ref(),
            Item::Implant(implant) => implant.get_sol_ref(),
            Item::Module(module) => module.get_sol_ref(),
            Item::ProjEffect(proj_effect) => proj_effect.get_sol_ref(),
            Item::Rig(rig) => rig.get_sol_ref(),
            Item::Service(service) => service.get_sol_ref(),
            Item::Ship(ship) => ship.get_sol_ref(),
            Item::Skill(skill) => skill.get_sol_ref(),
            Item::Stance(stance) => stance.get_sol_ref(),
            Item::Subsystem(subsystem) => subsystem.get_sol_ref(),
            Item::SwEffect(sw_effect) => sw_effect.get_sol_ref(),
        }
    }
}
impl<'s> ItemCommon for Item<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_sol(),
            Item::Booster(booster) => booster.get_sol(),
            Item::Character(character) => character.get_sol(),
            Item::Charge(charge) => charge.get_sol(),
            Item::Drone(drone) => drone.get_sol(),
            Item::Fighter(fighter) => fighter.get_sol(),
            Item::FwEffect(fw_effect) => fw_effect.get_sol(),
            Item::Implant(implant) => implant.get_sol(),
            Item::Module(module) => module.get_sol(),
            Item::ProjEffect(proj_effect) => proj_effect.get_sol(),
            Item::Rig(rig) => rig.get_sol(),
            Item::Service(service) => service.get_sol(),
            Item::Ship(ship) => ship.get_sol(),
            Item::Skill(skill) => skill.get_sol(),
            Item::Stance(stance) => stance.get_sol(),
            Item::Subsystem(subsystem) => subsystem.get_sol(),
            Item::SwEffect(sw_effect) => sw_effect.get_sol(),
        }
    }
}
