use crate::{
    Autocharge, Booster, Character, Charge, Drone, Fighter, FitId, FwEffect, Implant, ItemCommon, Module, ProjEffect,
    Rig, Service, Ship, Skill, Stance, Subsystem, SwEffect, api::ItemSealed, sol::SolCore, ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum Item<'sol> {
    Autocharge(Autocharge<'sol>),
    Booster(Booster<'sol>),
    Character(Character<'sol>),
    Charge(Charge<'sol>),
    Drone(Drone<'sol>),
    Fighter(Fighter<'sol>),
    FwEffect(FwEffect<'sol>),
    Implant(Implant<'sol>),
    Module(Module<'sol>),
    ProjEffect(ProjEffect<'sol>),
    Rig(Rig<'sol>),
    Service(Service<'sol>),
    Ship(Ship<'sol>),
    Skill(Skill<'sol>),
    Stance(Stance<'sol>),
    Subsystem(Subsystem<'sol>),
    SwEffect(SwEffect<'sol>),
}
impl<'sol> Item<'sol> {
    pub fn get_fit_id(&self) -> Option<FitId> {
        match self {
            Item::Autocharge(autocharge) => Some(autocharge.get_fit_id()),
            Item::Booster(booster) => Some(booster.get_fit_id()),
            Item::Character(character) => Some(character.get_fit_id()),
            Item::Charge(charge) => Some(charge.get_fit_id()),
            Item::Drone(drone) => Some(drone.get_fit_id()),
            Item::Fighter(fighter) => Some(fighter.get_fit_id()),
            Item::FwEffect(fw_effect) => Some(fw_effect.get_fit_id()),
            Item::Implant(implant) => Some(implant.get_fit_id()),
            Item::Module(module) => Some(module.get_fit_id()),
            Item::ProjEffect(..) => None,
            Item::Rig(rig) => Some(rig.get_fit_id()),
            Item::Service(service) => Some(service.get_fit_id()),
            Item::Ship(ship) => Some(ship.get_fit_id()),
            Item::Skill(skill) => Some(skill.get_fit_id()),
            Item::Stance(stance) => Some(stance.get_fit_id()),
            Item::Subsystem(subsystem) => Some(subsystem.get_fit_id()),
            Item::SwEffect(..) => None,
        }
    }
}
impl<'sol> ItemCommon for Item<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Item<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        sol.internal_get_item(uid)
    }
}
impl<'sol> ItemSealed for Item<'sol> {
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
    fn get_sol(&self) -> &SolCore {
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
    fn get_sol_mut(&mut self) -> &mut SolCore {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_sol_mut(),
            Item::Booster(booster) => booster.get_sol_mut(),
            Item::Character(character) => character.get_sol_mut(),
            Item::Charge(charge) => charge.get_sol_mut(),
            Item::Drone(drone) => drone.get_sol_mut(),
            Item::Fighter(fighter) => fighter.get_sol_mut(),
            Item::FwEffect(fw_effect) => fw_effect.get_sol_mut(),
            Item::Implant(implant) => implant.get_sol_mut(),
            Item::Module(module) => module.get_sol_mut(),
            Item::ProjEffect(proj_effect) => proj_effect.get_sol_mut(),
            Item::Rig(rig) => rig.get_sol_mut(),
            Item::Service(service) => service.get_sol_mut(),
            Item::Ship(ship) => ship.get_sol_mut(),
            Item::Skill(skill) => skill.get_sol_mut(),
            Item::Stance(stance) => stance.get_sol_mut(),
            Item::Subsystem(subsystem) => subsystem.get_sol_mut(),
            Item::SwEffect(sw_effect) => sw_effect.get_sol_mut(),
        }
    }
}
