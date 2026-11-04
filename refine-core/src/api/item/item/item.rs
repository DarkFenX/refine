use crate::{
    Autocharge, Booster, Character, Charge, Drone, Fighter, FitId, FwEffect, Implant, ItemCommon, Module, ProjEffect,
    Rig, Service, Ship, Skill, Stance, Subsystem, SwEffect, api::ItemSealed, sol::CoreInner, ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
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
impl<'s> ItemCommon for Item<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        core.internal_get_item(uid)
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
    fn get_core(&self) -> &CoreInner {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_core(),
            Item::Booster(booster) => booster.get_core(),
            Item::Character(character) => character.get_core(),
            Item::Charge(charge) => charge.get_core(),
            Item::Drone(drone) => drone.get_core(),
            Item::Fighter(fighter) => fighter.get_core(),
            Item::FwEffect(fw_effect) => fw_effect.get_core(),
            Item::Implant(implant) => implant.get_core(),
            Item::Module(module) => module.get_core(),
            Item::ProjEffect(proj_effect) => proj_effect.get_core(),
            Item::Rig(rig) => rig.get_core(),
            Item::Service(service) => service.get_core(),
            Item::Ship(ship) => ship.get_core(),
            Item::Skill(skill) => skill.get_core(),
            Item::Stance(stance) => stance.get_core(),
            Item::Subsystem(subsystem) => subsystem.get_core(),
            Item::SwEffect(sw_effect) => sw_effect.get_core(),
        }
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_core_mut(),
            Item::Booster(booster) => booster.get_core_mut(),
            Item::Character(character) => character.get_core_mut(),
            Item::Charge(charge) => charge.get_core_mut(),
            Item::Drone(drone) => drone.get_core_mut(),
            Item::Fighter(fighter) => fighter.get_core_mut(),
            Item::FwEffect(fw_effect) => fw_effect.get_core_mut(),
            Item::Implant(implant) => implant.get_core_mut(),
            Item::Module(module) => module.get_core_mut(),
            Item::ProjEffect(proj_effect) => proj_effect.get_core_mut(),
            Item::Rig(rig) => rig.get_core_mut(),
            Item::Service(service) => service.get_core_mut(),
            Item::Ship(ship) => ship.get_core_mut(),
            Item::Skill(skill) => skill.get_core_mut(),
            Item::Stance(stance) => stance.get_core_mut(),
            Item::Subsystem(subsystem) => subsystem.get_core_mut(),
            Item::SwEffect(sw_effect) => sw_effect.get_core_mut(),
        }
    }
}
