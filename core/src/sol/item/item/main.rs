use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    sol::item::{
        SolBooster, SolCharacter, SolCharge, SolDrone, SolEffectModes, SolFighter, SolFwEffect, SolImplant,
        SolItemState, SolModule, SolProjEffect, SolRig, SolShip, SolShipKind, SolSkill, SolStance, SolSubsystem,
        SolSwEffect,
    },
    src::Src,
    util::{Error, ErrorKind, Named, Result, StMap},
};

pub(in crate::sol) enum SolItem {
    Booster(SolBooster),
    Character(SolCharacter),
    Charge(SolCharge),
    Drone(SolDrone),
    Fighter(SolFighter),
    FwEffect(SolFwEffect),
    Implant(SolImplant),
    Module(SolModule),
    ProjEffect(SolProjEffect),
    Rig(SolRig),
    Ship(SolShip),
    Skill(SolSkill),
    Stance(SolStance),
    Subsystem(SolSubsystem),
    SwEffect(SolSwEffect),
}
impl SolItem {
    pub(in crate::sol) fn get_name(&self) -> &'static str {
        match self {
            Self::Booster(_) => SolBooster::get_name(),
            Self::Character(_) => SolCharacter::get_name(),
            Self::Charge(_) => SolCharge::get_name(),
            Self::Drone(_) => SolDrone::get_name(),
            Self::Fighter(_) => SolFighter::get_name(),
            Self::FwEffect(_) => SolFwEffect::get_name(),
            Self::Implant(_) => SolImplant::get_name(),
            Self::Module(_) => SolModule::get_name(),
            Self::ProjEffect(_) => SolProjEffect::get_name(),
            Self::Rig(_) => SolRig::get_name(),
            Self::Ship(_) => SolShip::get_name(),
            Self::Skill(_) => SolSkill::get_name(),
            Self::Stance(_) => SolStance::get_name(),
            Self::Subsystem(_) => SolSubsystem::get_name(),
            Self::SwEffect(_) => SolSwEffect::get_name(),
        }
    }
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        match self {
            Self::Booster(booster) => booster.base.id,
            Self::Character(character) => character.base.id,
            Self::Charge(charge) => charge.base.id,
            Self::Drone(drone) => drone.base.id,
            Self::Fighter(fighter) => fighter.base.id,
            Self::FwEffect(fw_effect) => fw_effect.base.id,
            Self::Implant(implant) => implant.base.id,
            Self::Module(module) => module.base.id,
            Self::ProjEffect(proj_effect) => proj_effect.base.id,
            Self::Rig(rig) => rig.base.id,
            Self::Ship(ship) => ship.base.id,
            Self::Skill(skill) => skill.base.id,
            Self::Stance(stance) => stance.base.id,
            Self::Subsystem(subsystem) => subsystem.base.id,
            Self::SwEffect(sw_effect) => sw_effect.base.id,
        }
    }
    pub(in crate::sol) fn get_fit_id(&self) -> Option<SolFitId> {
        match self {
            Self::Booster(booster) => Some(booster.fit_id),
            Self::Character(character) => Some(character.fit_id),
            Self::Charge(charge) => Some(charge.fit_id),
            Self::Drone(drone) => Some(drone.fit_id),
            Self::Fighter(fighter) => Some(fighter.fit_id),
            Self::FwEffect(fw_effect) => Some(fw_effect.fit_id),
            Self::Implant(implant) => Some(implant.fit_id),
            Self::Module(module) => Some(module.fit_id),
            Self::ProjEffect(_) => None,
            Self::Rig(rig) => Some(rig.fit_id),
            Self::Ship(ship) => Some(ship.fit_id),
            Self::Skill(skill) => Some(skill.fit_id),
            Self::Stance(stance) => Some(stance.fit_id),
            Self::Subsystem(subsystem) => Some(subsystem.fit_id),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        match self {
            Self::Booster(booster) => &booster.base.effect_modes,
            Self::Character(character) => &character.base.effect_modes,
            Self::Charge(charge) => &charge.base.effect_modes,
            Self::Drone(drone) => &drone.base.effect_modes,
            Self::Fighter(fighter) => &fighter.base.effect_modes,
            Self::FwEffect(fw_effect) => &fw_effect.base.effect_modes,
            Self::Implant(implant) => &implant.base.effect_modes,
            Self::Module(module) => &module.base.effect_modes,
            Self::ProjEffect(proj_effect) => &proj_effect.base.effect_modes,
            Self::Rig(rig) => &rig.base.effect_modes,
            Self::Ship(ship) => &ship.base.effect_modes,
            Self::Skill(skill) => &skill.base.effect_modes,
            Self::Stance(stance) => &stance.base.effect_modes,
            Self::Subsystem(subsystem) => &subsystem.base.effect_modes,
            Self::SwEffect(sw_effect) => &sw_effect.base.effect_modes,
        }
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        match self {
            Self::Booster(booster) => &mut booster.base.effect_modes,
            Self::Character(character) => &mut character.base.effect_modes,
            Self::Charge(charge) => &mut charge.base.effect_modes,
            Self::Drone(drone) => &mut drone.base.effect_modes,
            Self::Fighter(fighter) => &mut fighter.base.effect_modes,
            Self::FwEffect(fw_effect) => &mut fw_effect.base.effect_modes,
            Self::Implant(implant) => &mut implant.base.effect_modes,
            Self::Module(module) => &mut module.base.effect_modes,
            Self::ProjEffect(proj_effect) => &mut proj_effect.base.effect_modes,
            Self::Rig(rig) => &mut rig.base.effect_modes,
            Self::Ship(ship) => &mut ship.base.effect_modes,
            Self::Skill(skill) => &mut skill.base.effect_modes,
            Self::Stance(stance) => &mut stance.base.effect_modes,
            Self::Subsystem(subsystem) => &mut subsystem.base.effect_modes,
            Self::SwEffect(sw_effect) => &mut sw_effect.base.effect_modes,
        }
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> EItemId {
        match self {
            Self::Booster(booster) => booster.base.a_item_id,
            Self::Character(character) => character.base.a_item_id,
            Self::Charge(charge) => charge.base.a_item_id,
            Self::Drone(drone) => drone.base.a_item_id,
            Self::Fighter(fighter) => fighter.base.a_item_id,
            Self::FwEffect(fw_effect) => fw_effect.base.a_item_id,
            Self::Implant(implant) => implant.base.a_item_id,
            Self::Module(module) => module.base.a_item_id,
            Self::ProjEffect(proj_effect) => proj_effect.base.a_item_id,
            Self::Rig(rig) => rig.base.a_item_id,
            Self::Ship(ship) => ship.base.a_item_id,
            Self::Skill(skill) => skill.base.a_item_id,
            Self::Stance(stance) => stance.base.a_item_id,
            Self::Subsystem(subsystem) => subsystem.base.a_item_id,
            Self::SwEffect(sw_effect) => sw_effect.base.a_item_id,
        }
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        match self {
            Self::Booster(booster) => booster.state,
            Self::Character(character) => character.state,
            Self::Charge(_) => SolItemState::Offline,
            Self::Drone(drone) => drone.state,
            Self::Fighter(fighter) => fighter.state,
            Self::FwEffect(fw_effect) => fw_effect.state,
            Self::Implant(implant) => implant.state,
            Self::Module(module) => module.state,
            Self::ProjEffect(proj_effect) => proj_effect.state,
            Self::Rig(rig) => rig.state,
            Self::Ship(ship) => ship.state,
            Self::Skill(skill) => skill.state,
            Self::Stance(stance) => stance.state,
            Self::Subsystem(subsystem) => subsystem.state,
            Self::SwEffect(sw_effect) => sw_effect.state,
        }
    }
    pub(in crate::sol) fn reload_a_item(&mut self, src: &Src) {
        let a_item_id = self.get_a_item_id();
        let a_item = src.get_a_item(&a_item_id).cloned();
        match self {
            Self::Booster(booster) => booster.base.a_item = a_item,
            Self::Character(character) => character.base.a_item = a_item,
            Self::Charge(charge) => charge.base.a_item = a_item,
            Self::Drone(drone) => drone.base.a_item = a_item,
            Self::Fighter(fighter) => fighter.base.a_item = a_item,
            Self::FwEffect(fw_effect) => fw_effect.base.a_item = a_item,
            Self::Implant(implant) => implant.base.a_item = a_item,
            Self::Module(module) => module.base.a_item = a_item,
            Self::ProjEffect(proj_effect) => proj_effect.base.a_item = a_item,
            Self::Rig(rig) => rig.base.a_item = a_item,
            Self::Ship(ship) => ship.base.a_item = a_item,
            Self::Skill(skill) => skill.base.a_item = a_item,
            Self::Stance(stance) => stance.base.a_item = a_item,
            Self::Subsystem(subsystem) => subsystem.base.a_item = a_item,
            Self::SwEffect(sw_effect) => sw_effect.base.a_item = a_item,
        }
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        match self {
            Self::Booster(booster) => booster.base.a_item.as_ref(),
            Self::Character(character) => character.base.a_item.as_ref(),
            Self::Charge(charge) => charge.base.a_item.as_ref(),
            Self::Drone(drone) => drone.base.a_item.as_ref(),
            Self::Fighter(fighter) => fighter.base.a_item.as_ref(),
            Self::FwEffect(fw_effect) => fw_effect.base.a_item.as_ref(),
            Self::Implant(implant) => implant.base.a_item.as_ref(),
            Self::Module(module) => module.base.a_item.as_ref(),
            Self::ProjEffect(proj_effect) => proj_effect.base.a_item.as_ref(),
            Self::Rig(rig) => rig.base.a_item.as_ref(),
            Self::Ship(ship) => ship.base.a_item.as_ref(),
            Self::Skill(skill) => skill.base.a_item.as_ref(),
            Self::Stance(stance) => stance.base.a_item.as_ref(),
            Self::Subsystem(subsystem) => subsystem.base.a_item.as_ref(),
            Self::SwEffect(sw_effect) => sw_effect.base.a_item.as_ref(),
        }
        .ok_or_else(|| Error::new(ErrorKind::AItemNotLoaded(self.get_a_item_id())))
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.get_a_item().is_ok()
    }
    pub(in crate::sol) fn can_receive_projs(&self) -> bool {
        match self {
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => false,
            Self::Drone(_) => true,
            Self::Fighter(_) => true,
            Self::FwEffect(_) => false,
            Self::Implant(_) => false,
            Self::Module(_) => false,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => false,
            Self::Ship(_) => true,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol) fn iter_projs(&self) -> Option<impl ExactSizeIterator<Item = (&SolItemId, &Option<AttrVal>)>> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => Some(module.projs.iter()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.projs.iter()),
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn iter_projectee_items(&self) -> Option<impl ExactSizeIterator<Item = &SolItemId>> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => Some(module.projs.iter_items()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.projs.iter_items()),
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    // Calculator-specific getters
    // TODO: consider moving to calculator specific item extensions
    pub(in crate::sol) fn get_orig_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>> {
        Ok(&self.get_a_item()?.attr_vals)
    }
    pub(in crate::sol) fn get_effect_datas(&self) -> Result<&StMap<EEffectId, ad::AItemEffectData>> {
        Ok(&self.get_a_item()?.effect_datas)
    }
    pub(in crate::sol) fn get_defeff_id(&self) -> Result<&Option<EEffectId>> {
        Ok(&self.get_a_item()?.defeff_id)
    }
    pub(in crate::sol) fn get_group_id(&self) -> Result<EItemGrpId> {
        Ok(self.get_a_item()?.grp_id)
    }
    pub(in crate::sol) fn get_category_id(&self) -> Result<EItemCatId> {
        Ok(self.get_a_item()?.cat_id)
    }
    pub(in crate::sol) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>> {
        Ok(&self.get_a_item()?.srqs)
    }
    pub(in crate::sol) fn get_ship_kind(&self) -> SolShipKind {
        match self {
            Self::Ship(ship) => ship.kind,
            _ => SolShipKind::default(),
        }
    }
}
