use std::{collections::HashMap, sync::Arc};

pub(in crate::ss) use booster::Booster;
pub(in crate::ss) use character::Character;
pub(in crate::ss) use charge::Charge;
pub(in crate::ss) use drone::Drone;
pub(in crate::ss) use fighter::Fighter;
pub(in crate::ss) use implant::Implant;
pub(in crate::ss) use module::Module;
pub(in crate::ss) use rig::Rig;
pub(in crate::ss) use ship::Ship;
pub(in crate::ss) use skill::Skill;
pub(in crate::ss) use stance::Stance;
pub(in crate::ss) use subsystem::Subsystem;
pub(in crate::ss) use sw_effect::SwEffect;

use crate::{
    consts::{ModDomain, State},
    ct, ReeFloat, ReeId, ReeInt, Src,
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

pub(in crate::ss) enum Item {
    Booster(Booster),
    Character(Character),
    Charge(Charge),
    Drone(Drone),
    Fighter(Fighter),
    Implant(Implant),
    ModuleHigh(Module),
    ModuleLow(Module),
    ModuleMid(Module),
    Rig(Rig),
    Ship(Ship),
    Skill(Skill),
    Stance(Stance),
    Subsystem(Subsystem),
    SwEffect(SwEffect),
}
impl Item {
    pub(in crate::ss) fn get_id(&self) -> ReeId {
        match self {
            Item::Booster(i) => i.item_id,
            Item::Character(i) => i.item_id,
            Item::Charge(i) => i.item_id,
            Item::Drone(i) => i.item_id,
            Item::Fighter(i) => i.item_id,
            Item::Implant(i) => i.item_id,
            Item::ModuleHigh(i) => i.item_id,
            Item::ModuleLow(i) => i.item_id,
            Item::ModuleMid(i) => i.item_id,
            Item::Rig(i) => i.item_id,
            Item::Ship(i) => i.item_id,
            Item::Skill(i) => i.item_id,
            Item::Stance(i) => i.item_id,
            Item::Subsystem(i) => i.item_id,
            Item::SwEffect(i) => i.item_id,
        }
    }
    pub(in crate::ss) fn get_fit_id(&self) -> Option<ReeId> {
        match self {
            Item::Booster(i) => Some(i.fit_id),
            Item::Character(i) => Some(i.fit_id),
            Item::Charge(i) => Some(i.fit_id),
            Item::Drone(i) => Some(i.fit_id),
            Item::Fighter(i) => Some(i.fit_id),
            Item::Implant(i) => Some(i.fit_id),
            Item::ModuleHigh(i) => Some(i.fit_id),
            Item::ModuleLow(i) => Some(i.fit_id),
            Item::ModuleMid(i) => Some(i.fit_id),
            Item::Rig(i) => Some(i.fit_id),
            Item::Ship(i) => Some(i.fit_id),
            Item::Skill(i) => Some(i.fit_id),
            Item::Stance(i) => Some(i.fit_id),
            Item::Subsystem(i) => Some(i.fit_id),
            Item::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_type_id(&self) -> ReeInt {
        match self {
            Item::Booster(i) => i.type_id,
            Item::Character(i) => i.type_id,
            Item::Charge(i) => i.type_id,
            Item::Drone(i) => i.type_id,
            Item::Fighter(i) => i.type_id,
            Item::Implant(i) => i.type_id,
            Item::ModuleHigh(i) => i.type_id,
            Item::ModuleLow(i) => i.type_id,
            Item::ModuleMid(i) => i.type_id,
            Item::Rig(i) => i.type_id,
            Item::Ship(i) => i.type_id,
            Item::Skill(i) => i.type_id,
            Item::Stance(i) => i.type_id,
            Item::Subsystem(i) => i.type_id,
            Item::SwEffect(i) => i.type_id,
        }
    }
    pub(in crate::ss) fn get_state(&self) -> State {
        match self {
            Item::Booster(i) => i.state,
            Item::Character(_) => State::Offline,
            Item::Charge(_) => State::Offline,
            Item::Drone(i) => i.state,
            Item::Fighter(i) => i.state,
            Item::Implant(i) => i.state,
            Item::ModuleHigh(i) => i.state,
            Item::ModuleLow(i) => i.state,
            Item::ModuleMid(i) => i.state,
            Item::Rig(i) => i.state,
            Item::Ship(_) => State::Offline,
            Item::Skill(_) => State::Offline,
            Item::Stance(_) => State::Offline,
            Item::Subsystem(_) => State::Offline,
            Item::SwEffect(_) => State::Offline,
        }
    }
    pub(in crate::ss) fn reload_cached_item(&mut self, src: &Arc<Src>) {
        let type_id = self.get_type_id();
        let cached_item = src.cache_handler.get_item(&type_id);
        match self {
            Item::Booster(i) => i.citem = cached_item,
            Item::Character(i) => i.citem = cached_item,
            Item::Charge(i) => i.citem = cached_item,
            Item::Drone(i) => i.citem = cached_item,
            Item::Fighter(i) => i.citem = cached_item,
            Item::Implant(i) => i.citem = cached_item,
            Item::ModuleHigh(i) => i.citem = cached_item,
            Item::ModuleLow(i) => i.citem = cached_item,
            Item::ModuleMid(i) => i.citem = cached_item,
            Item::Rig(i) => i.citem = cached_item,
            Item::Ship(i) => i.citem = cached_item,
            Item::Skill(i) => i.citem = cached_item,
            Item::Stance(i) => i.citem = cached_item,
            Item::Subsystem(i) => i.citem = cached_item,
            Item::SwEffect(i) => i.citem = cached_item,
        }
    }
    pub(in crate::ss) fn get_citem(&self) -> Option<&Arc<ct::Item>> {
        match self {
            Item::Booster(i) => i.citem.as_ref(),
            Item::Character(i) => i.citem.as_ref(),
            Item::Charge(i) => i.citem.as_ref(),
            Item::Drone(i) => i.citem.as_ref(),
            Item::Fighter(i) => i.citem.as_ref(),
            Item::Implant(i) => i.citem.as_ref(),
            Item::ModuleHigh(i) => i.citem.as_ref(),
            Item::ModuleLow(i) => i.citem.as_ref(),
            Item::ModuleMid(i) => i.citem.as_ref(),
            Item::Rig(i) => i.citem.as_ref(),
            Item::Ship(i) => i.citem.as_ref(),
            Item::Skill(i) => i.citem.as_ref(),
            Item::Stance(i) => i.citem.as_ref(),
            Item::Subsystem(i) => i.citem.as_ref(),
            Item::SwEffect(i) => i.citem.as_ref(),
        }
    }
    pub(in crate::ss) fn is_loaded(&self) -> bool {
        self.get_citem().is_some()
    }
    // Calculator-specific getters
    pub(in crate::ss) fn get_orig_attrs(&self) -> Option<&HashMap<ReeInt, ReeFloat>> {
        self.get_citem().map(|v| &v.attr_vals)
    }
    pub(in crate::ss) fn get_effect_datas(&self) -> Option<&HashMap<ReeInt, ct::ItemEffData>> {
        self.get_citem().map(|v| &v.effect_datas)
    }
    pub(in crate::ss) fn get_top_domain(&self) -> Option<ModDomain> {
        match self {
            Item::Booster(_) => None,
            Item::Character(_) => Some(ModDomain::Char),
            Item::Charge(_) => None,
            Item::Drone(_) => None,
            Item::Fighter(_) => None,
            Item::Implant(_) => None,
            Item::ModuleHigh(_) => None,
            Item::ModuleLow(_) => None,
            Item::ModuleMid(_) => None,
            Item::Rig(_) => None,
            Item::Ship(_) => Some(ModDomain::Ship),
            Item::Skill(_) => None,
            Item::Stance(_) => None,
            Item::Subsystem(_) => None,
            Item::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_parent_domain(&self) -> Option<ModDomain> {
        match self {
            Item::Booster(_) => Some(ModDomain::Char),
            Item::Character(_) => None,
            Item::Charge(_) => Some(ModDomain::Ship),
            Item::Drone(_) => None,
            Item::Fighter(_) => None,
            Item::Implant(_) => Some(ModDomain::Char),
            Item::ModuleHigh(_) => Some(ModDomain::Ship),
            Item::ModuleLow(_) => Some(ModDomain::Ship),
            Item::ModuleMid(_) => Some(ModDomain::Ship),
            Item::Rig(_) => Some(ModDomain::Ship),
            Item::Ship(_) => None,
            Item::Skill(_) => Some(ModDomain::Char),
            Item::Stance(_) => Some(ModDomain::Ship),
            Item::Subsystem(_) => Some(ModDomain::Ship),
            Item::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_group_id(&self) -> Option<ReeInt> {
        self.get_citem().map(|v| v.grp_id)
    }
    pub(in crate::ss) fn get_skill_reqs(&self) -> Option<&HashMap<ReeInt, ReeInt>> {
        self.get_citem().map(|v| &v.srqs)
    }
    pub(in crate::ss) fn is_owner_modifiable(&self) -> bool {
        match self {
            Item::Booster(_) => false,
            Item::Character(_) => false,
            Item::Charge(_) => true,
            Item::Drone(_) => true,
            Item::Fighter(_) => true,
            Item::Implant(_) => false,
            Item::ModuleHigh(_) => false,
            Item::ModuleLow(_) => false,
            Item::ModuleMid(_) => false,
            Item::Rig(_) => false,
            Item::Ship(_) => false,
            Item::Skill(_) => false,
            Item::Stance(_) => false,
            Item::Subsystem(_) => false,
            Item::SwEffect(_) => false,
        }
    }
}
