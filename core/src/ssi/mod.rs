use std::{collections::HashMap, sync::Arc};

pub(crate) use booster::SsBooster;
pub(crate) use character::SsCharacter;
pub(crate) use charge::SsCharge;
pub(crate) use drone::SsDrone;
pub(crate) use fighter::SsFighter;
pub(crate) use implant::SsImplant;
pub(crate) use module::SsModule;
pub(crate) use rig::SsRig;
pub(crate) use ship::SsShip;
pub(crate) use skill::SsSkill;
pub(crate) use stance::SsStance;
pub(crate) use subsystem::SsSubsystem;
pub(crate) use sw_effect::SsSwEffect;

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

pub(crate) enum SsItem {
    Booster(SsBooster),
    Character(SsCharacter),
    Charge(SsCharge),
    Drone(SsDrone),
    Fighter(SsFighter),
    Implant(SsImplant),
    Module(SsModule),
    Rig(SsRig),
    Ship(SsShip),
    Skill(SsSkill),
    Stance(SsStance),
    Subsystem(SsSubsystem),
    SwEffect(SsSwEffect),
}
impl SsItem {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            Self::Booster(_) => SsBooster::get_name(),
            Self::Character(_) => SsCharacter::get_name(),
            Self::Charge(_) => SsCharge::get_name(),
            Self::Drone(_) => SsDrone::get_name(),
            Self::Fighter(_) => SsFighter::get_name(),
            Self::Implant(_) => SsImplant::get_name(),
            Self::Module(_) => SsModule::get_name(),
            Self::Rig(_) => SsRig::get_name(),
            Self::Ship(_) => SsShip::get_name(),
            Self::Skill(_) => SsSkill::get_name(),
            Self::Stance(_) => SsStance::get_name(),
            Self::Subsystem(_) => SsSubsystem::get_name(),
            Self::SwEffect(_) => SsSwEffect::get_name(),
        }
    }
    pub(crate) fn get_id(&self) -> ReeId {
        match self {
            Self::Booster(ss_booster) => ss_booster.id,
            Self::Character(ss_character) => ss_character.id,
            Self::Charge(ss_charge) => ss_charge.id,
            Self::Drone(ss_drone) => ss_drone.id,
            Self::Fighter(ss_fighter) => ss_fighter.id,
            Self::Implant(ss_implant) => ss_implant.id,
            Self::Module(ss_module) => ss_module.id,
            Self::Rig(ss_rig) => ss_rig.id,
            Self::Ship(ss_ship) => ss_ship.id,
            Self::Skill(ss_skill) => ss_skill.id,
            Self::Stance(ss_stance) => ss_stance.id,
            Self::Subsystem(ss_subsystem) => ss_subsystem.id,
            Self::SwEffect(ss_sw_effect) => ss_sw_effect.id,
        }
    }
    pub(crate) fn get_fit_id(&self) -> Option<ReeId> {
        match self {
            Self::Booster(ss_booster) => Some(ss_booster.fit_id),
            Self::Character(ss_character) => Some(ss_character.fit_id),
            Self::Charge(ss_charge) => Some(ss_charge.fit_id),
            Self::Drone(ss_drone) => Some(ss_drone.fit_id),
            Self::Fighter(ss_fighter) => Some(ss_fighter.fit_id),
            Self::Implant(ss_implant) => Some(ss_implant.fit_id),
            Self::Module(ss_module) => Some(ss_module.fit_id),
            Self::Rig(ss_rig) => Some(ss_rig.fit_id),
            Self::Ship(ss_ship) => Some(ss_ship.fit_id),
            Self::Skill(ss_skill) => Some(ss_skill.fit_id),
            Self::Stance(ss_stance) => Some(ss_stance.fit_id),
            Self::Subsystem(ss_subsystem) => Some(ss_subsystem.fit_id),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_a_item_id(&self) -> ReeInt {
        match self {
            Self::Booster(ss_booster) => ss_booster.a_item_id,
            Self::Character(ss_character) => ss_character.a_item_id,
            Self::Charge(ss_charge) => ss_charge.a_item_id,
            Self::Drone(ss_drone) => ss_drone.a_item_id,
            Self::Fighter(ss_fighter) => ss_fighter.a_item_id,
            Self::Implant(ss_implant) => ss_implant.a_item_id,
            Self::Module(ss_module) => ss_module.a_item_id,
            Self::Rig(ss_rig) => ss_rig.a_item_id,
            Self::Ship(ss_ship) => ss_ship.a_item_id,
            Self::Skill(ss_skill) => ss_skill.a_item_id,
            Self::Stance(ss_stance) => ss_stance.a_item_id,
            Self::Subsystem(ss_subsystem) => ss_subsystem.a_item_id,
            Self::SwEffect(ss_sw_effect) => ss_sw_effect.a_item_id,
        }
    }
    pub(crate) fn get_state(&self) -> State {
        match self {
            Self::Booster(ss_booster) => ss_booster.state,
            Self::Character(ss_character) => ss_character.state,
            Self::Charge(ss_charge) => State::Offline,
            Self::Drone(ss_drone) => ss_drone.state,
            Self::Fighter(ss_fighter) => ss_fighter.state,
            Self::Implant(ss_implant) => ss_implant.state,
            Self::Module(ss_module) => ss_module.state,
            Self::Rig(ss_rig) => ss_rig.state,
            Self::Ship(ss_ship) => ss_ship.state,
            Self::Skill(ss_skill) => ss_skill.state,
            Self::Stance(ss_stance) => ss_stance.state,
            Self::Subsystem(ss_subsystem) => ss_subsystem.state,
            Self::SwEffect(ss_sw_effect) => ss_sw_effect.state,
        }
    }
    pub(crate) fn reload_a_item(&mut self, src: &Arc<Src>) {
        let a_item_id = self.get_a_item_id();
        let a_item = src.a_handler.get_item(&a_item_id);
        match self {
            Self::Booster(ss_booster) => ss_booster.a_item = a_item,
            Self::Character(ss_character) => ss_character.a_item = a_item,
            Self::Charge(ss_charge) => ss_charge.a_item = a_item,
            Self::Drone(ss_drone) => ss_drone.a_item = a_item,
            Self::Fighter(ss_fighter) => ss_fighter.a_item = a_item,
            Self::Implant(ss_implant) => ss_implant.a_item = a_item,
            Self::Module(ss_module) => ss_module.a_item = a_item,
            Self::Rig(ss_rig) => ss_rig.a_item = a_item,
            Self::Ship(ss_ship) => ss_ship.a_item = a_item,
            Self::Skill(ss_skill) => ss_skill.a_item = a_item,
            Self::Stance(ss_stance) => ss_stance.a_item = a_item,
            Self::Subsystem(ss_subsystem) => ss_subsystem.a_item = a_item,
            Self::SwEffect(ss_sw_effect) => ss_sw_effect.a_item = a_item,
        }
    }
    pub(crate) fn get_a_item(&self) -> Result<&Arc<ad::AItem>> {
        match self {
            Self::Booster(ss_booster) => ss_booster.a_item.as_ref(),
            Self::Character(ss_character) => ss_character.a_item.as_ref(),
            Self::Charge(ss_charge) => ss_charge.a_item.as_ref(),
            Self::Drone(ss_drone) => ss_drone.a_item.as_ref(),
            Self::Fighter(ss_fighter) => ss_fighter.a_item.as_ref(),
            Self::Implant(ss_implant) => ss_implant.a_item.as_ref(),
            Self::Module(ss_module) => ss_module.a_item.as_ref(),
            Self::Rig(ss_rig) => ss_rig.a_item.as_ref(),
            Self::Ship(ss_ship) => ss_ship.a_item.as_ref(),
            Self::Skill(ss_skill) => ss_skill.a_item.as_ref(),
            Self::Stance(ss_stance) => ss_stance.a_item.as_ref(),
            Self::Subsystem(ss_subsystem) => ss_subsystem.a_item.as_ref(),
            Self::SwEffect(ss_sw_effect) => ss_sw_effect.a_item.as_ref(),
        }
        .ok_or_else(|| Error::new(ErrorKind::AItemNotLoaded(self.get_a_item_id())))
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.get_a_item().is_ok()
    }
    // Calculator-specific getters
    pub(crate) fn get_orig_attrs(&self) -> Result<&HashMap<ReeInt, ReeFloat>> {
        self.get_a_item().map(|v| &v.attr_vals)
    }
    pub(crate) fn get_effect_datas(&self) -> Result<&HashMap<ReeInt, ad::AItemEffData>> {
        self.get_a_item().map(|v| &v.effect_datas)
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
        self.get_a_item().map(|v| v.grp_id)
    }
    pub(crate) fn get_category_id(&self) -> Result<ReeInt> {
        self.get_a_item().map(|v| v.cat_id)
    }
    pub(crate) fn get_skill_reqs(&self) -> Result<&HashMap<ReeInt, ReeInt>> {
        self.get_a_item().map(|v| &v.srqs)
    }
    pub(crate) fn get_other(&self) -> Option<ReeId> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(ss_charge) => Some(ss_charge.cont_id),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::Module(ss_module) => ss_module.charge_a_item_id,
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
