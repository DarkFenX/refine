use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    sol::item::{
        SolAutoCharge, SolAutocharges, SolBooster, SolCharacter, SolCharge, SolDrone, SolEffectModes, SolFighter,
        SolFwEffect, SolImplant, SolItemState, SolModule, SolProjEffect, SolRig, SolShip, SolShipKind, SolSkill,
        SolStance, SolSubsystem, SolSwEffect,
    },
    src::Src,
    util::{Named, Result, StMap},
};

pub(in crate::sol) enum SolItem {
    AutoCharge(SolAutoCharge),
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
            Self::AutoCharge(_) => SolAutoCharge::get_name(),
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
            Self::AutoCharge(autocharge) => autocharge.get_id(),
            Self::Booster(booster) => booster.get_id(),
            Self::Character(character) => character.get_id(),
            Self::Charge(charge) => charge.get_id(),
            Self::Drone(drone) => drone.get_id(),
            Self::Fighter(fighter) => fighter.get_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_id(),
            Self::Implant(implant) => implant.get_id(),
            Self::Module(module) => module.get_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_id(),
            Self::Rig(rig) => rig.get_id(),
            Self::Ship(ship) => ship.get_id(),
            Self::Skill(skill) => skill.get_id(),
            Self::Stance(stance) => stance.get_id(),
            Self::Subsystem(subsystem) => subsystem.get_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_id(),
        }
    }
    pub(in crate::sol) fn get_fit_id(&self) -> Option<SolFitId> {
        match self {
            Self::AutoCharge(autocharge) => Some(autocharge.get_fit_id()),
            Self::Booster(booster) => Some(booster.get_fit_id()),
            Self::Character(character) => Some(character.get_fit_id()),
            Self::Charge(charge) => Some(charge.get_fit_id()),
            Self::Drone(drone) => Some(drone.get_fit_id()),
            Self::Fighter(fighter) => Some(fighter.get_fit_id()),
            Self::FwEffect(fw_effect) => Some(fw_effect.get_fit_id()),
            Self::Implant(implant) => Some(implant.get_fit_id()),
            Self::Module(module) => Some(module.get_fit_id()),
            Self::ProjEffect(_) => None,
            Self::Rig(rig) => Some(rig.get_fit_id()),
            Self::Ship(ship) => Some(ship.get_fit_id()),
            Self::Skill(skill) => Some(skill.get_fit_id()),
            Self::Stance(stance) => Some(stance.get_fit_id()),
            Self::Subsystem(subsystem) => Some(subsystem.get_fit_id()),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        match self {
            Self::AutoCharge(autocharge) => autocharge.get_effect_modes(),
            Self::Booster(booster) => booster.get_effect_modes(),
            Self::Character(character) => character.get_effect_modes(),
            Self::Charge(charge) => charge.get_effect_modes(),
            Self::Drone(drone) => drone.get_effect_modes(),
            Self::Fighter(fighter) => fighter.get_effect_modes(),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_modes(),
            Self::Implant(implant) => implant.get_effect_modes(),
            Self::Module(module) => module.get_effect_modes(),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_modes(),
            Self::Rig(rig) => rig.get_effect_modes(),
            Self::Ship(ship) => ship.get_effect_modes(),
            Self::Skill(skill) => skill.get_effect_modes(),
            Self::Stance(stance) => stance.get_effect_modes(),
            Self::Subsystem(subsystem) => subsystem.get_effect_modes(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_modes(),
        }
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        match self {
            Self::AutoCharge(autocharge) => autocharge.get_effect_modes_mut(),
            Self::Booster(booster) => booster.get_effect_modes_mut(),
            Self::Character(character) => character.get_effect_modes_mut(),
            Self::Charge(charge) => charge.get_effect_modes_mut(),
            Self::Drone(drone) => drone.get_effect_modes_mut(),
            Self::Fighter(fighter) => fighter.get_effect_modes_mut(),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_modes_mut(),
            Self::Implant(implant) => implant.get_effect_modes_mut(),
            Self::Module(module) => module.get_effect_modes_mut(),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_modes_mut(),
            Self::Rig(rig) => rig.get_effect_modes_mut(),
            Self::Ship(ship) => ship.get_effect_modes_mut(),
            Self::Skill(skill) => skill.get_effect_modes_mut(),
            Self::Stance(stance) => stance.get_effect_modes_mut(),
            Self::Subsystem(subsystem) => subsystem.get_effect_modes_mut(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_modes_mut(),
        }
    }
    pub(in crate::sol) fn get_autocharges(&self) -> Option<&SolAutocharges> {
        match self {
            Self::AutoCharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(fighter) => Some(&fighter.autocharges),
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(_) => None,
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_autocharges_mut(&mut self) -> Option<&mut SolAutocharges> {
        match self {
            Self::AutoCharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(fighter) => Some(&mut fighter.autocharges),
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(_) => None,
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> EItemId {
        match self {
            Self::AutoCharge(autocharge) => autocharge.get_a_item_id(),
            Self::Booster(booster) => booster.get_a_item_id(),
            Self::Character(character) => character.get_a_item_id(),
            Self::Charge(charge) => charge.get_a_item_id(),
            Self::Drone(drone) => drone.get_a_item_id(),
            Self::Fighter(fighter) => fighter.get_a_item_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_item_id(),
            Self::Implant(implant) => implant.get_a_item_id(),
            Self::Module(module) => module.get_a_item_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_item_id(),
            Self::Rig(rig) => rig.get_a_item_id(),
            Self::Ship(ship) => ship.get_a_item_id(),
            Self::Skill(skill) => skill.get_a_item_id(),
            Self::Stance(stance) => stance.get_a_item_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_item_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_item_id(),
        }
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        match self {
            Self::AutoCharge(_) => SolItemState::Offline,
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
        match self {
            Self::AutoCharge(autocharge) => autocharge.reload_a_item(src),
            Self::Booster(booster) => booster.reload_a_item(src),
            Self::Character(character) => character.reload_a_item(src),
            Self::Charge(charge) => charge.reload_a_item(src),
            Self::Drone(drone) => drone.reload_a_item(src),
            Self::Fighter(fighter) => fighter.reload_a_item(src),
            Self::FwEffect(fw_effect) => fw_effect.reload_a_item(src),
            Self::Implant(implant) => implant.reload_a_item(src),
            Self::Module(module) => module.reload_a_item(src),
            Self::ProjEffect(proj_effect) => proj_effect.reload_a_item(src),
            Self::Rig(rig) => rig.reload_a_item(src),
            Self::Ship(ship) => ship.reload_a_item(src),
            Self::Skill(skill) => skill.reload_a_item(src),
            Self::Stance(stance) => stance.reload_a_item(src),
            Self::Subsystem(subsystem) => subsystem.reload_a_item(src),
            Self::SwEffect(sw_effect) => sw_effect.reload_a_item(src),
        }
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        match self {
            Self::AutoCharge(autocharge) => autocharge.get_a_item(),
            Self::Booster(booster) => booster.get_a_item(),
            Self::Character(character) => character.get_a_item(),
            Self::Charge(charge) => charge.get_a_item(),
            Self::Drone(drone) => drone.get_a_item(),
            Self::Fighter(fighter) => fighter.get_a_item(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_item(),
            Self::Implant(implant) => implant.get_a_item(),
            Self::Module(module) => module.get_a_item(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_item(),
            Self::Rig(rig) => rig.get_a_item(),
            Self::Ship(ship) => ship.get_a_item(),
            Self::Skill(skill) => skill.get_a_item(),
            Self::Stance(stance) => stance.get_a_item(),
            Self::Subsystem(subsystem) => subsystem.get_a_item(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_item(),
        }
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        match self {
            Self::AutoCharge(autocharge) => autocharge.is_loaded(),
            Self::Booster(booster) => booster.is_loaded(),
            Self::Character(character) => character.is_loaded(),
            Self::Charge(charge) => charge.is_loaded(),
            Self::Drone(drone) => drone.is_loaded(),
            Self::Fighter(fighter) => fighter.is_loaded(),
            Self::FwEffect(fw_effect) => fw_effect.is_loaded(),
            Self::Implant(implant) => implant.is_loaded(),
            Self::Module(module) => module.is_loaded(),
            Self::ProjEffect(proj_effect) => proj_effect.is_loaded(),
            Self::Rig(rig) => rig.is_loaded(),
            Self::Ship(ship) => ship.is_loaded(),
            Self::Skill(skill) => skill.is_loaded(),
            Self::Stance(stance) => stance.is_loaded(),
            Self::Subsystem(subsystem) => subsystem.is_loaded(),
            Self::SwEffect(sw_effect) => sw_effect.is_loaded(),
        }
    }
    pub(in crate::sol) fn can_receive_projs(&self) -> bool {
        match self {
            Self::AutoCharge(_) => false,
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
            Self::AutoCharge(_) => None,
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
            Self::AutoCharge(_) => None,
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
