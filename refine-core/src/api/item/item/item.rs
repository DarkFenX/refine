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
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        sol.internal_get_item(uid)
    }
}
impl<'s> ItemSealed for Item<'s> {
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
impl<'s> ItemCommon for Item<'s> {}

pub enum ItemMut<'s> {
    Autocharge(AutochargeMut<'s>),
    Booster(BoosterMut<'s>),
    Character(CharacterMut<'s>),
    Charge(ChargeMut<'s>),
    Drone(DroneMut<'s>),
    Fighter(FighterMut<'s>),
    FwEffect(FwEffectMut<'s>),
    Implant(ImplantMut<'s>),
    Module(ModuleMut<'s>),
    ProjEffect(ProjEffectMut<'s>),
    Rig(RigMut<'s>),
    Service(ServiceMut<'s>),
    Ship(ShipMut<'s>),
    Skill(SkillMut<'s>),
    Stance(StanceMut<'s>),
    Subsystem(SubsystemMut<'s>),
    SwEffect(SwEffectMut<'s>),
}
impl<'s> ItemMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        sol.internal_get_item_mut(uid)
    }
}
impl<'s> ItemSealed for ItemMut<'s> {
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
impl<'s> ItemMutSealed for ItemMut<'s> {
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
impl<'s> ItemCommon for ItemMut<'s> {}
impl<'s> ItemMutCommon for ItemMut<'s> {}
