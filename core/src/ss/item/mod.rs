use std::{collections::HashMap, sync::Arc};

pub(in crate::ss) use booster::{Booster, BoosterInfo};
pub(in crate::ss) use character::{Character, CharacterInfo};
pub(in crate::ss) use charge::{Charge, ChargeInfo};
pub(in crate::ss) use drone::{Drone, DroneInfo};
pub(in crate::ss) use fighter::{Fighter, FighterInfo};
pub(in crate::ss) use implant::{Implant, ImplantInfo};
pub(in crate::ss) use module::{Module, ModuleInfo};
pub(in crate::ss) use rig::{Rig, RigInfo};
pub(in crate::ss) use ship::{Ship, ShipInfo};
pub(in crate::ss) use skill::{Skill, SkillInfo};
pub(in crate::ss) use stance::{Stance, StanceInfo};
pub(in crate::ss) use subsystem::{Subsystem, SubsystemInfo};
pub(in crate::ss) use sw_effect::{SwEffect, SwEffectInfo};

use crate::{
    consts::{ModDomain, State},
    ct, ReeFloat, ReeId, ReeIdx, ReeInt, Src,
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
            Self::Booster(i) => i.item_id,
            Self::Character(i) => i.item_id,
            Self::Charge(i) => i.item_id,
            Self::Drone(i) => i.item_id,
            Self::Fighter(i) => i.item_id,
            Self::Implant(i) => i.item_id,
            Self::ModuleHigh(i) => i.item_id,
            Self::ModuleLow(i) => i.item_id,
            Self::ModuleMid(i) => i.item_id,
            Self::Rig(i) => i.item_id,
            Self::Ship(i) => i.item_id,
            Self::Skill(i) => i.item_id,
            Self::Stance(i) => i.item_id,
            Self::Subsystem(i) => i.item_id,
            Self::SwEffect(i) => i.item_id,
        }
    }
    pub(in crate::ss) fn get_fit_id(&self) -> Option<ReeId> {
        match self {
            Self::Booster(i) => Some(i.fit_id),
            Self::Character(i) => Some(i.fit_id),
            Self::Charge(i) => Some(i.fit_id),
            Self::Drone(i) => Some(i.fit_id),
            Self::Fighter(i) => Some(i.fit_id),
            Self::Implant(i) => Some(i.fit_id),
            Self::ModuleHigh(i) => Some(i.fit_id),
            Self::ModuleLow(i) => Some(i.fit_id),
            Self::ModuleMid(i) => Some(i.fit_id),
            Self::Rig(i) => Some(i.fit_id),
            Self::Ship(i) => Some(i.fit_id),
            Self::Skill(i) => Some(i.fit_id),
            Self::Stance(i) => Some(i.fit_id),
            Self::Subsystem(i) => Some(i.fit_id),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_type_id(&self) -> ReeInt {
        match self {
            Self::Booster(i) => i.type_id,
            Self::Character(i) => i.type_id,
            Self::Charge(i) => i.type_id,
            Self::Drone(i) => i.type_id,
            Self::Fighter(i) => i.type_id,
            Self::Implant(i) => i.type_id,
            Self::ModuleHigh(i) => i.type_id,
            Self::ModuleLow(i) => i.type_id,
            Self::ModuleMid(i) => i.type_id,
            Self::Rig(i) => i.type_id,
            Self::Ship(i) => i.type_id,
            Self::Skill(i) => i.type_id,
            Self::Stance(i) => i.type_id,
            Self::Subsystem(i) => i.type_id,
            Self::SwEffect(i) => i.type_id,
        }
    }
    pub(in crate::ss) fn get_state(&self) -> State {
        match self {
            Self::Booster(i) => i.state,
            Self::Character(_) => State::Offline,
            Self::Charge(_) => State::Offline,
            Self::Drone(i) => i.state,
            Self::Fighter(i) => i.state,
            Self::Implant(i) => i.state,
            Self::ModuleHigh(i) => i.state,
            Self::ModuleLow(i) => i.state,
            Self::ModuleMid(i) => i.state,
            Self::Rig(i) => i.state,
            Self::Ship(_) => State::Offline,
            Self::Skill(_) => State::Offline,
            Self::Stance(_) => State::Offline,
            Self::Subsystem(_) => State::Offline,
            Self::SwEffect(_) => State::Offline,
        }
    }
    pub(in crate::ss) fn reload_cached_item(&mut self, src: &Arc<Src>) {
        let type_id = self.get_type_id();
        let cached_item = src.cache_handler.get_item(&type_id);
        match self {
            Self::Booster(i) => i.citem = cached_item,
            Self::Character(i) => i.citem = cached_item,
            Self::Charge(i) => i.citem = cached_item,
            Self::Drone(i) => i.citem = cached_item,
            Self::Fighter(i) => i.citem = cached_item,
            Self::Implant(i) => i.citem = cached_item,
            Self::ModuleHigh(i) => i.citem = cached_item,
            Self::ModuleLow(i) => i.citem = cached_item,
            Self::ModuleMid(i) => i.citem = cached_item,
            Self::Rig(i) => i.citem = cached_item,
            Self::Ship(i) => i.citem = cached_item,
            Self::Skill(i) => i.citem = cached_item,
            Self::Stance(i) => i.citem = cached_item,
            Self::Subsystem(i) => i.citem = cached_item,
            Self::SwEffect(i) => i.citem = cached_item,
        }
    }
    pub(in crate::ss) fn get_citem(&self) -> Option<&Arc<ct::Item>> {
        match self {
            Self::Booster(i) => i.citem.as_ref(),
            Self::Character(i) => i.citem.as_ref(),
            Self::Charge(i) => i.citem.as_ref(),
            Self::Drone(i) => i.citem.as_ref(),
            Self::Fighter(i) => i.citem.as_ref(),
            Self::Implant(i) => i.citem.as_ref(),
            Self::ModuleHigh(i) => i.citem.as_ref(),
            Self::ModuleLow(i) => i.citem.as_ref(),
            Self::ModuleMid(i) => i.citem.as_ref(),
            Self::Rig(i) => i.citem.as_ref(),
            Self::Ship(i) => i.citem.as_ref(),
            Self::Skill(i) => i.citem.as_ref(),
            Self::Stance(i) => i.citem.as_ref(),
            Self::Subsystem(i) => i.citem.as_ref(),
            Self::SwEffect(i) => i.citem.as_ref(),
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
            Self::Booster(_) => None,
            Self::Character(_) => Some(ModDomain::Char),
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::ModuleHigh(_) => None,
            Self::ModuleLow(_) => None,
            Self::ModuleMid(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => Some(ModDomain::Ship),
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_parent_domain(&self) -> Option<ModDomain> {
        match self {
            Self::Booster(_) => Some(ModDomain::Char),
            Self::Character(_) => None,
            Self::Charge(_) => Some(ModDomain::Ship),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => Some(ModDomain::Char),
            Self::ModuleHigh(_) => Some(ModDomain::Ship),
            Self::ModuleLow(_) => Some(ModDomain::Ship),
            Self::ModuleMid(_) => Some(ModDomain::Ship),
            Self::Rig(_) => Some(ModDomain::Ship),
            Self::Ship(_) => None,
            Self::Skill(_) => Some(ModDomain::Char),
            Self::Stance(_) => Some(ModDomain::Ship),
            Self::Subsystem(_) => Some(ModDomain::Ship),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn get_group_id(&self) -> Option<ReeInt> {
        self.get_citem().map(|v| v.grp_id)
    }
    pub(in crate::ss) fn get_category_id(&self) -> Option<ReeInt> {
        self.get_citem().map(|v| v.cat_id)
    }
    pub(in crate::ss) fn get_skill_reqs(&self) -> Option<&HashMap<ReeInt, ReeInt>> {
        self.get_citem().map(|v| &v.srqs)
    }
    pub(in crate::ss) fn get_other(&self) -> Option<ReeId> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(i) => Some(i.cont),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::ModuleHigh(i) => i.charge,
            Self::ModuleLow(i) => i.charge,
            Self::ModuleMid(i) => i.charge,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::ss) fn is_owner_modifiable(&self) -> bool {
        match self {
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => true,
            Self::Drone(_) => true,
            Self::Fighter(_) => true,
            Self::Implant(_) => false,
            Self::ModuleHigh(_) => false,
            Self::ModuleLow(_) => false,
            Self::ModuleMid(_) => false,
            Self::Rig(_) => false,
            Self::Ship(_) => false,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
}
