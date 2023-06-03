use std::{collections::HashMap, sync::Arc};

pub(crate) use booster::Booster;
pub(crate) use character::Character;
pub(crate) use charge::Charge;
pub(crate) use drone::Drone;
pub(crate) use fighter::Fighter;
pub(crate) use implant::Implant;
pub(crate) use module::Module;
pub(crate) use rig::Rig;
pub(crate) use ship::Ship;
pub(crate) use skill::Skill;
pub(crate) use stance::Stance;
pub(crate) use subsystem::Subsystem;
pub(crate) use sw_effect::SwEffect;

use crate::{
    ad,
    consts::{ModDomain, State},
    defs::{ReeFloat, ReeId, ReeInt},
    src::Src,
    util::{Error, ErrorKind, Named, Result},
};

mod booster;
mod character;
mod charge;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;
mod sw_effect;

pub(crate) enum Item {
    Booster(Booster),
    Character(Character),
    Charge(Charge),
    Drone(Drone),
    Fighter(Fighter),
    Implant(Implant),
    Module(Module),
    Rig(Rig),
    Ship(Ship),
    Skill(Skill),
    Stance(Stance),
    Subsystem(Subsystem),
    SwEffect(SwEffect),
}
impl Item {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            Self::Booster(_) => Booster::get_name(),
            Self::Character(_) => Character::get_name(),
            Self::Charge(_) => Charge::get_name(),
            Self::Drone(_) => Drone::get_name(),
            Self::Fighter(_) => Fighter::get_name(),
            Self::Implant(_) => Implant::get_name(),
            Self::Module(_) => Module::get_name(),
            Self::Rig(_) => Rig::get_name(),
            Self::Ship(_) => Ship::get_name(),
            Self::Skill(_) => Skill::get_name(),
            Self::Stance(_) => Stance::get_name(),
            Self::Subsystem(_) => Subsystem::get_name(),
            Self::SwEffect(_) => SwEffect::get_name(),
        }
    }
    pub(crate) fn get_id(&self) -> ReeId {
        match self {
            Self::Booster(booster) => booster.id,
            Self::Character(character) => character.id,
            Self::Charge(charge) => charge.id,
            Self::Drone(drone) => drone.id,
            Self::Fighter(fighter) => fighter.id,
            Self::Implant(implant) => implant.id,
            Self::Module(module) => module.id,
            Self::Rig(rig) => rig.id,
            Self::Ship(ship) => ship.id,
            Self::Skill(skill) => skill.id,
            Self::Stance(stance) => stance.id,
            Self::Subsystem(subsystem) => subsystem.id,
            Self::SwEffect(sw_effect) => sw_effect.id,
        }
    }
    pub(crate) fn get_fit_id(&self) -> Option<ReeId> {
        match self {
            Self::Booster(booster) => Some(booster.fit_id),
            Self::Character(character) => Some(character.fit_id),
            Self::Charge(charge) => Some(charge.fit_id),
            Self::Drone(drone) => Some(drone.fit_id),
            Self::Fighter(fighter) => Some(fighter.fit_id),
            Self::Implant(implant) => Some(implant.fit_id),
            Self::Module(module) => Some(module.fit_id),
            Self::Rig(rig) => Some(rig.fit_id),
            Self::Ship(ship) => Some(ship.fit_id),
            Self::Skill(skill) => Some(skill.fit_id),
            Self::Stance(stance) => Some(stance.fit_id),
            Self::Subsystem(subsystem) => Some(subsystem.fit_id),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_type_id(&self) -> ReeInt {
        match self {
            Self::Booster(booster) => booster.type_id,
            Self::Character(character) => character.type_id,
            Self::Charge(charge) => charge.type_id,
            Self::Drone(drone) => drone.type_id,
            Self::Fighter(fighter) => fighter.type_id,
            Self::Implant(implant) => implant.type_id,
            Self::Module(module) => module.type_id,
            Self::Rig(rig) => rig.type_id,
            Self::Ship(ship) => ship.type_id,
            Self::Skill(skill) => skill.type_id,
            Self::Stance(stance) => stance.type_id,
            Self::Subsystem(subsystem) => subsystem.type_id,
            Self::SwEffect(sw_effect) => sw_effect.type_id,
        }
    }
    pub(crate) fn get_state(&self) -> State {
        match self {
            Self::Booster(booster) => booster.state,
            Self::Character(character) => character.state,
            Self::Charge(charge) => State::Offline,
            Self::Drone(drone) => drone.state,
            Self::Fighter(fighter) => fighter.state,
            Self::Implant(implant) => implant.state,
            Self::Module(module) => module.state,
            Self::Rig(rig) => rig.state,
            Self::Ship(ship) => ship.state,
            Self::Skill(skill) => skill.state,
            Self::Stance(stance) => stance.state,
            Self::Subsystem(subsystem) => subsystem.state,
            Self::SwEffect(sw_effect) => sw_effect.state,
        }
    }
    pub(crate) fn reload_cached_item(&mut self, src: &Arc<Src>) {
        let type_id = self.get_type_id();
        let cached_item = src.cache_handler.get_item(&type_id);
        match self {
            Self::Booster(booster) => booster.cached_item = cached_item,
            Self::Character(character) => character.cached_item = cached_item,
            Self::Charge(charge) => charge.cached_item = cached_item,
            Self::Drone(drone) => drone.cached_item = cached_item,
            Self::Fighter(fighter) => fighter.cached_item = cached_item,
            Self::Implant(implant) => implant.cached_item = cached_item,
            Self::Module(module) => module.cached_item = cached_item,
            Self::Rig(rig) => rig.cached_item = cached_item,
            Self::Ship(ship) => ship.cached_item = cached_item,
            Self::Skill(skill) => skill.cached_item = cached_item,
            Self::Stance(stance) => stance.cached_item = cached_item,
            Self::Subsystem(subsystem) => subsystem.cached_item = cached_item,
            Self::SwEffect(sw_effect) => sw_effect.cached_item = cached_item,
        }
    }
    pub(crate) fn get_cached_item(&self) -> Result<&Arc<ad::AItem>> {
        match self {
            Self::Booster(booster) => booster.cached_item.as_ref(),
            Self::Character(character) => character.cached_item.as_ref(),
            Self::Charge(charge) => charge.cached_item.as_ref(),
            Self::Drone(drone) => drone.cached_item.as_ref(),
            Self::Fighter(fighter) => fighter.cached_item.as_ref(),
            Self::Implant(implant) => implant.cached_item.as_ref(),
            Self::Module(module) => module.cached_item.as_ref(),
            Self::Rig(rig) => rig.cached_item.as_ref(),
            Self::Ship(ship) => ship.cached_item.as_ref(),
            Self::Skill(skill) => skill.cached_item.as_ref(),
            Self::Stance(stance) => stance.cached_item.as_ref(),
            Self::Subsystem(subsystem) => subsystem.cached_item.as_ref(),
            Self::SwEffect(sw_effect) => sw_effect.cached_item.as_ref(),
        }
        .ok_or_else(|| Error::new(ErrorKind::CachedItemNotLoaded(self.get_type_id())))
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.get_cached_item().is_ok()
    }
    // Calculator-specific getters
    pub(crate) fn get_orig_attrs(&self) -> Result<&HashMap<ReeInt, ReeFloat>> {
        self.get_cached_item().map(|v| &v.attr_vals)
    }
    pub(crate) fn get_effect_datas(&self) -> Result<&HashMap<ReeInt, ad::AItemEffData>> {
        self.get_cached_item().map(|v| &v.effect_datas)
    }
    pub(crate) fn get_top_domain(&self) -> Option<ModDomain> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => Some(ModDomain::Char),
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::Module(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => Some(ModDomain::Ship),
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_parent_domain(&self) -> Option<ModDomain> {
        match self {
            Self::Booster(_) => Some(ModDomain::Char),
            Self::Character(_) => None,
            Self::Charge(_) => Some(ModDomain::Ship),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => Some(ModDomain::Char),
            Self::Module(_) => Some(ModDomain::Ship),
            Self::Rig(_) => Some(ModDomain::Ship),
            Self::Ship(_) => None,
            Self::Skill(_) => Some(ModDomain::Char),
            Self::Stance(_) => Some(ModDomain::Ship),
            Self::Subsystem(_) => Some(ModDomain::Ship),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_group_id(&self) -> Result<ReeInt> {
        self.get_cached_item().map(|v| v.grp_id)
    }
    pub(crate) fn get_category_id(&self) -> Result<ReeInt> {
        self.get_cached_item().map(|v| v.cat_id)
    }
    pub(crate) fn get_skill_reqs(&self) -> Result<&HashMap<ReeInt, ReeInt>> {
        self.get_cached_item().map(|v| &v.srqs)
    }
    pub(crate) fn get_other(&self) -> Option<ReeId> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.cont_id),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => module.charge_id,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn is_owner_modifiable(&self) -> bool {
        match self {
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => true,
            Self::Drone(_) => true,
            Self::Fighter(_) => true,
            Self::Implant(_) => false,
            Self::Module(_) => false,
            Self::Rig(_) => false,
            Self::Ship(_) => false,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
}

fn bool_to_state(bool_state: bool) -> State {
    match bool_state {
        true => State::Offline,
        false => State::Ghost,
    }
}

fn state_to_bool(state: State) -> bool {
    match state {
        State::Ghost => false,
        _ => true,
    }
}
