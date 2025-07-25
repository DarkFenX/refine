use crate::{
    sol::{
        SolarSystem,
        api::{
            Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone,
            DroneMut, Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, ItemCommon, ItemMutCommon,
            ItemMutSealed, ItemSealed, Module, ModuleMut, ProjEffect, ProjEffectMut, Rig, RigMut, Service, ServiceMut,
            Ship, ShipMut, Skill, SkillMut, Stance, StanceMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
        },
    },
    ud::UItemKey,
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
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        sol.internal_get_item(key)
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
    fn get_key(&self) -> UItemKey {
        match self {
            Item::Autocharge(autocharge) => autocharge.get_key(),
            Item::Booster(booster) => booster.get_key(),
            Item::Character(character) => character.get_key(),
            Item::Charge(charge) => charge.get_key(),
            Item::Drone(drone) => drone.get_key(),
            Item::Fighter(fighter) => fighter.get_key(),
            Item::FwEffect(fw_effect) => fw_effect.get_key(),
            Item::Implant(implant) => implant.get_key(),
            Item::Module(module) => module.get_key(),
            Item::ProjEffect(proj_effect) => proj_effect.get_key(),
            Item::Rig(rig) => rig.get_key(),
            Item::Service(service) => service.get_key(),
            Item::Ship(ship) => ship.get_key(),
            Item::Skill(skill) => skill.get_key(),
            Item::Stance(stance) => stance.get_key(),
            Item::Subsystem(subsystem) => subsystem.get_key(),
            Item::SwEffect(sw_effect) => sw_effect.get_key(),
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
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        sol.internal_get_item_mut(key)
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
    fn get_key(&self) -> UItemKey {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.get_key(),
            ItemMut::Booster(booster) => booster.get_key(),
            ItemMut::Character(character) => character.get_key(),
            ItemMut::Charge(charge) => charge.get_key(),
            ItemMut::Drone(drone) => drone.get_key(),
            ItemMut::Fighter(fighter) => fighter.get_key(),
            ItemMut::FwEffect(fw_effect) => fw_effect.get_key(),
            ItemMut::Implant(implant) => implant.get_key(),
            ItemMut::Module(module) => module.get_key(),
            ItemMut::ProjEffect(proj_effect) => proj_effect.get_key(),
            ItemMut::Rig(rig) => rig.get_key(),
            ItemMut::Service(service) => service.get_key(),
            ItemMut::Ship(ship) => ship.get_key(),
            ItemMut::Skill(skill) => skill.get_key(),
            ItemMut::Stance(stance) => stance.get_key(),
            ItemMut::Subsystem(subsystem) => subsystem.get_key(),
            ItemMut::SwEffect(sw_effect) => sw_effect.get_key(),
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
