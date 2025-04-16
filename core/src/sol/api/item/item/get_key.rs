use crate::sol::{
    ItemKey,
    api::{Item, ItemMut},
};

impl<'a> Item<'a> {
    pub(in crate::sol) fn get_key(&self) -> ItemKey {
        match self {
            Item::Autocharge(autocharge) => autocharge.key,
            Item::Booster(booster) => booster.key,
            Item::Character(character) => character.key,
            Item::Charge(charge) => charge.key,
            Item::Drone(drone) => drone.key,
            Item::Fighter(fighter) => fighter.key,
            Item::FwEffect(fw_effect) => fw_effect.key,
            Item::Implant(implant) => implant.key,
            Item::Module(module) => module.key,
            Item::ProjEffect(proj_effect) => proj_effect.key,
            Item::Rig(rig) => rig.key,
            Item::Service(service) => service.key,
            Item::Ship(ship) => ship.key,
            Item::Skill(skill) => skill.key,
            Item::Stance(stance) => stance.key,
            Item::Subsystem(subsystem) => subsystem.key,
            Item::SwEffect(sw_effect) => sw_effect.key,
        }
    }
}

impl<'a> ItemMut<'a> {
    pub(in crate::sol) fn get_key(&self) -> ItemKey {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.key,
            ItemMut::Booster(booster) => booster.key,
            ItemMut::Character(character) => character.key,
            ItemMut::Charge(charge) => charge.key,
            ItemMut::Drone(drone) => drone.key,
            ItemMut::Fighter(fighter) => fighter.key,
            ItemMut::FwEffect(fw_effect) => fw_effect.key,
            ItemMut::Implant(implant) => implant.key,
            ItemMut::Module(module) => module.key,
            ItemMut::ProjEffect(proj_effect) => proj_effect.key,
            ItemMut::Rig(rig) => rig.key,
            ItemMut::Service(service) => service.key,
            ItemMut::Ship(ship) => ship.key,
            ItemMut::Skill(skill) => skill.key,
            ItemMut::Stance(stance) => stance.key,
            ItemMut::Subsystem(subsystem) => subsystem.key,
            ItemMut::SwEffect(sw_effect) => sw_effect.key,
        }
    }
}
