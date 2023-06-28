use std::collections::HashMap;

pub(in crate::ss) use booster::SsBooster;
pub(in crate::ss) use character::SsCharacter;
pub(in crate::ss) use charge::SsCharge;
pub(in crate::ss) use drone::SsDrone;
pub(in crate::ss) use fighter::SsFighter;
pub(in crate::ss) use implant::SsImplant;
pub(in crate::ss) use module::SsModule;
pub(in crate::ss) use rig::SsRig;
pub(in crate::ss) use ship::SsShip;
pub(in crate::ss) use skill::SsSkill;
pub(in crate::ss) use stance::SsStance;
pub(in crate::ss) use subsystem::SsSubsystem;
pub(in crate::ss) use sw_effect::SsSwEffect;

use crate::{
    ad,
    consts::{EffectMode, ModDomain, State},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SsFitId, SsItemId},
    src::Src,
    util::{Error, ErrorKind, Named, OptMap, Result},
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

pub(in crate::ss) enum SsItem {
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
    pub(in crate::ss) fn get_name(&self) -> &'static str {
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
    pub(in crate::ss) fn get_id(&self) -> SsItemId {
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
    pub(in crate::ss) fn get_fit_id(&self) -> Option<SsFitId> {
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
    pub(in crate::ss) fn get_effect_modes(&self) -> &OptMap<EEffectId, EffectMode> {
        match self {
            Self::Booster(booster) => &booster.effect_modes,
            Self::Character(character) => &character.effect_modes,
            Self::Charge(charge) => &charge.effect_modes,
            Self::Drone(drone) => &drone.effect_modes,
            Self::Fighter(fighter) => &fighter.effect_modes,
            Self::Implant(implant) => &implant.effect_modes,
            Self::Module(module) => &module.effect_modes,
            Self::Rig(rig) => &rig.effect_modes,
            Self::Ship(ship) => &ship.effect_modes,
            Self::Skill(skill) => &skill.effect_modes,
            Self::Stance(stance) => &stance.effect_modes,
            Self::Subsystem(subsystem) => &subsystem.effect_modes,
            Self::SwEffect(sw_effect) => &sw_effect.effect_modes,
        }
    }
    pub(in crate::ss) fn get_effect_modes_mut(&mut self) -> &mut OptMap<EEffectId, EffectMode> {
        match self {
            Self::Booster(booster) => &mut booster.effect_modes,
            Self::Character(character) => &mut character.effect_modes,
            Self::Charge(charge) => &mut charge.effect_modes,
            Self::Drone(drone) => &mut drone.effect_modes,
            Self::Fighter(fighter) => &mut fighter.effect_modes,
            Self::Implant(implant) => &mut implant.effect_modes,
            Self::Module(module) => &mut module.effect_modes,
            Self::Rig(rig) => &mut rig.effect_modes,
            Self::Ship(ship) => &mut ship.effect_modes,
            Self::Skill(skill) => &mut skill.effect_modes,
            Self::Stance(stance) => &mut stance.effect_modes,
            Self::Subsystem(subsystem) => &mut subsystem.effect_modes,
            Self::SwEffect(sw_effect) => &mut sw_effect.effect_modes,
        }
    }
    pub(in crate::ss) fn get_a_item_id(&self) -> EItemId {
        match self {
            Self::Booster(booster) => booster.a_item_id,
            Self::Character(character) => character.a_item_id,
            Self::Charge(charge) => charge.a_item_id,
            Self::Drone(drone) => drone.a_item_id,
            Self::Fighter(fighter) => fighter.a_item_id,
            Self::Implant(implant) => implant.a_item_id,
            Self::Module(module) => module.a_item_id,
            Self::Rig(rig) => rig.a_item_id,
            Self::Ship(ship) => ship.a_item_id,
            Self::Skill(skill) => skill.a_item_id,
            Self::Stance(stance) => stance.a_item_id,
            Self::Subsystem(subsystem) => subsystem.a_item_id,
            Self::SwEffect(sw_effect) => sw_effect.a_item_id,
        }
    }
    pub(in crate::ss) fn get_state(&self) -> State {
        match self {
            Self::Booster(booster) => booster.state,
            Self::Character(character) => character.state,
            Self::Charge(_) => State::Offline,
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
    pub(in crate::ss) fn reload_a_item(&mut self, src: &Src) {
        let a_item_id = self.get_a_item_id();
        let a_item = src.get_a_item(&a_item_id);
        match self {
            Self::Booster(booster) => booster.a_item = a_item,
            Self::Character(character) => character.a_item = a_item,
            Self::Charge(charge) => charge.a_item = a_item,
            Self::Drone(drone) => drone.a_item = a_item,
            Self::Fighter(fighter) => fighter.a_item = a_item,
            Self::Implant(implant) => implant.a_item = a_item,
            Self::Module(module) => module.a_item = a_item,
            Self::Rig(rig) => rig.a_item = a_item,
            Self::Ship(ship) => ship.a_item = a_item,
            Self::Skill(skill) => skill.a_item = a_item,
            Self::Stance(stance) => stance.a_item = a_item,
            Self::Subsystem(subsystem) => subsystem.a_item = a_item,
            Self::SwEffect(sw_effect) => sw_effect.a_item = a_item,
        }
    }
    pub(in crate::ss) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        match self {
            Self::Booster(booster) => booster.a_item.as_ref(),
            Self::Character(character) => character.a_item.as_ref(),
            Self::Charge(charge) => charge.a_item.as_ref(),
            Self::Drone(drone) => drone.a_item.as_ref(),
            Self::Fighter(fighter) => fighter.a_item.as_ref(),
            Self::Implant(implant) => implant.a_item.as_ref(),
            Self::Module(module) => module.a_item.as_ref(),
            Self::Rig(rig) => rig.a_item.as_ref(),
            Self::Ship(ship) => ship.a_item.as_ref(),
            Self::Skill(skill) => skill.a_item.as_ref(),
            Self::Stance(stance) => stance.a_item.as_ref(),
            Self::Subsystem(subsystem) => subsystem.a_item.as_ref(),
            Self::SwEffect(sw_effect) => sw_effect.a_item.as_ref(),
        }
        .ok_or_else(|| Error::new(ErrorKind::AItemNotLoaded(self.get_a_item_id())))
    }
    pub(in crate::ss) fn is_loaded(&self) -> bool {
        self.get_a_item().is_ok()
    }
    // Calculator-specific getters
    pub(in crate::ss) fn get_orig_attrs(&self) -> Result<&HashMap<EAttrId, AttrVal>> {
        Ok(&self.get_a_item()?.attr_vals)
    }
    pub(in crate::ss) fn get_effect_datas(&self) -> Result<&HashMap<EEffectId, ad::AItemEffData>> {
        Ok(&self.get_a_item()?.effect_datas)
    }
    pub(in crate::ss) fn get_top_domain(&self) -> Option<ModDomain> {
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
    pub(in crate::ss) fn get_parent_domain(&self) -> Option<ModDomain> {
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
    pub(in crate::ss) fn get_group_id(&self) -> Result<EItemGrpId> {
        Ok(self.get_a_item()?.grp_id)
    }
    pub(in crate::ss) fn get_category_id(&self) -> Result<EItemCatId> {
        Ok(self.get_a_item()?.cat_id)
    }
    pub(in crate::ss) fn get_skill_reqs(&self) -> Result<&HashMap<EItemId, SkillLevel>> {
        Ok(&self.get_a_item()?.srqs)
    }
    pub(in crate::ss) fn get_other(&self) -> Option<SsItemId> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.cont_id),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => module.charge_a_item_id,
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
