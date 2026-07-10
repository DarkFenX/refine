use crate::{
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, ItemCommon, ItemMutCommon, ItemMutSealed,
        ItemSealed, Module, ModuleMut, ProjEffect, ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut,
        Skill, SkillMut, Stance, StanceMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
    },
    sol::SolarSystem,
    ud::UItemId,
};

pub enum Item<'a> {
    Autocharge(Autocharge<'a>),
    Booster(Booster<'a>),
    Character(Character<'a>),
    Charge(Charge<'a>),
    Drone(Drone<'a>),
    Fighter(Fighter<'a>),
    FwEffect(FwEffect<'a>),
    Implant(Implant<'a>),
    Module(Module<'a>),
    ProjEffect(ProjEffect<'a>),
    Rig(Rig<'a>),
    Service(Service<'a>),
    Ship(Ship<'a>),
    Skill(Skill<'a>),
    Stance(Stance<'a>),
    Subsystem(Subsystem<'a>),
    SwEffect(SwEffect<'a>),
}
impl<'a> Item<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        sol.internal_get_item(uid)
    }
}
impl<'a> ItemSealed for Item<'a> {
    fn get_sol(&self) -> &SolarSystem {
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
}
impl<'a> ItemCommon for Item<'a> {}

pub enum ItemMut<'a> {
    Autocharge(AutochargeMut<'a>),
    Booster(BoosterMut<'a>),
    Character(CharacterMut<'a>),
    Charge(ChargeMut<'a>),
    Drone(DroneMut<'a>),
    Fighter(FighterMut<'a>),
    FwEffect(FwEffectMut<'a>),
    Implant(ImplantMut<'a>),
    Module(ModuleMut<'a>),
    ProjEffect(ProjEffectMut<'a>),
    Rig(RigMut<'a>),
    Service(ServiceMut<'a>),
    Ship(ShipMut<'a>),
    Skill(SkillMut<'a>),
    Stance(StanceMut<'a>),
    Subsystem(SubsystemMut<'a>),
    SwEffect(SwEffectMut<'a>),
}
impl<'a> ItemMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        sol.internal_get_item_mut(uid)
    }
}
impl<'a> ItemSealed for ItemMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.get_sol(),
            ItemMut::Booster(booster) => booster.get_sol(),
            ItemMut::Character(character) => character.get_sol(),
            ItemMut::Charge(charge) => charge.get_sol(),
            ItemMut::Drone(drone) => drone.get_sol(),
            ItemMut::Fighter(fighter) => fighter.get_sol(),
            ItemMut::FwEffect(fw_effect) => fw_effect.get_sol(),
            ItemMut::Implant(implant) => implant.get_sol(),
            ItemMut::Module(module) => module.get_sol(),
            ItemMut::ProjEffect(proj_effect) => proj_effect.get_sol(),
            ItemMut::Rig(rig) => rig.get_sol(),
            ItemMut::Service(service) => service.get_sol(),
            ItemMut::Ship(ship) => ship.get_sol(),
            ItemMut::Skill(skill) => skill.get_sol(),
            ItemMut::Stance(stance) => stance.get_sol(),
            ItemMut::Subsystem(subsystem) => subsystem.get_sol(),
            ItemMut::SwEffect(sw_effect) => sw_effect.get_sol(),
        }
    }
    fn get_uid(&self) -> UItemId {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.get_uid(),
            ItemMut::Booster(booster) => booster.get_uid(),
            ItemMut::Character(character) => character.get_uid(),
            ItemMut::Charge(charge) => charge.get_uid(),
            ItemMut::Drone(drone) => drone.get_uid(),
            ItemMut::Fighter(fighter) => fighter.get_uid(),
            ItemMut::FwEffect(fw_effect) => fw_effect.get_uid(),
            ItemMut::Implant(implant) => implant.get_uid(),
            ItemMut::Module(module) => module.get_uid(),
            ItemMut::ProjEffect(proj_effect) => proj_effect.get_uid(),
            ItemMut::Rig(rig) => rig.get_uid(),
            ItemMut::Service(service) => service.get_uid(),
            ItemMut::Ship(ship) => ship.get_uid(),
            ItemMut::Skill(skill) => skill.get_uid(),
            ItemMut::Stance(stance) => stance.get_uid(),
            ItemMut::Subsystem(subsystem) => subsystem.get_uid(),
            ItemMut::SwEffect(sw_effect) => sw_effect.get_uid(),
        }
    }
}
impl<'a> ItemMutSealed for ItemMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.get_sol_mut(),
            ItemMut::Booster(booster) => booster.get_sol_mut(),
            ItemMut::Character(character) => character.get_sol_mut(),
            ItemMut::Charge(charge) => charge.get_sol_mut(),
            ItemMut::Drone(drone) => drone.get_sol_mut(),
            ItemMut::Fighter(fighter) => fighter.get_sol_mut(),
            ItemMut::FwEffect(fw_effect) => fw_effect.get_sol_mut(),
            ItemMut::Implant(implant) => implant.get_sol_mut(),
            ItemMut::Module(module) => module.get_sol_mut(),
            ItemMut::ProjEffect(proj_effect) => proj_effect.get_sol_mut(),
            ItemMut::Rig(rig) => rig.get_sol_mut(),
            ItemMut::Service(service) => service.get_sol_mut(),
            ItemMut::Ship(ship) => ship.get_sol_mut(),
            ItemMut::Skill(skill) => skill.get_sol_mut(),
            ItemMut::Stance(stance) => stance.get_sol_mut(),
            ItemMut::Subsystem(subsystem) => subsystem.get_sol_mut(),
            ItemMut::SwEffect(sw_effect) => sw_effect.get_sol_mut(),
        }
    }
}
impl<'a> ItemCommon for ItemMut<'a> {}
impl<'a> ItemMutCommon for ItemMut<'a> {}
