use either::Either;

use crate::{
    ad::{AAttrVal, AEffectId, AItemCatId, AItemGrpId, AItemId, ASkillLevel, AState},
    def::{AttrVal, ItemId, OF},
    misc::{EffectMode, Spool},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, Src},
    ud::{
        UAutocharge, UBooster, UCharacter, UCharge, UData, UDrone, UFighter, UFitId, UFwEffect, UImplant, UItemId,
        UModule, UPhysics, UProjEffect, URig, UService, UShip, USkill, UStance, USubsystem, USwEffect,
        item::{ItemMutationData, UAutocharges, UEffectUpdates, UProjData, UProjs},
    },
    util::{GetId, Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) enum UItem {
    Autocharge(UAutocharge),
    Booster(UBooster),
    Character(UCharacter),
    Charge(UCharge),
    Drone(UDrone),
    Fighter(UFighter),
    FwEffect(UFwEffect),
    Implant(UImplant),
    Module(UModule),
    ProjEffect(UProjEffect),
    Service(UService),
    Rig(URig),
    Ship(UShip),
    Skill(USkill),
    Stance(UStance),
    Subsystem(USubsystem),
    SwEffect(USwEffect),
}
impl UItem {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            Self::Autocharge(_) => UAutocharge::get_name(),
            Self::Booster(_) => UBooster::get_name(),
            Self::Character(_) => UCharacter::get_name(),
            Self::Charge(_) => UCharge::get_name(),
            Self::Drone(_) => UDrone::get_name(),
            Self::Fighter(_) => UFighter::get_name(),
            Self::FwEffect(_) => UFwEffect::get_name(),
            Self::Implant(_) => UImplant::get_name(),
            Self::Module(_) => UModule::get_name(),
            Self::ProjEffect(_) => UProjEffect::get_name(),
            Self::Rig(_) => URig::get_name(),
            Self::Service(_) => UService::get_name(),
            Self::Ship(_) => UShip::get_name(),
            Self::Skill(_) => USkill::get_name(),
            Self::Stance(_) => UStance::get_name(),
            Self::Subsystem(_) => USubsystem::get_name(),
            Self::SwEffect(_) => USwEffect::get_name(),
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Access to base item methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn get_item_id(&self) -> ItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_item_id(),
            Self::Booster(booster) => booster.get_item_id(),
            Self::Character(character) => character.get_item_id(),
            Self::Charge(charge) => charge.get_item_id(),
            Self::Drone(drone) => drone.get_item_id(),
            Self::Fighter(fighter) => fighter.get_item_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_item_id(),
            Self::Implant(implant) => implant.get_item_id(),
            Self::Module(module) => module.get_item_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_item_id(),
            Self::Rig(rig) => rig.get_item_id(),
            Self::Service(service) => service.get_item_id(),
            Self::Ship(ship) => ship.get_item_id(),
            Self::Skill(skill) => skill.get_item_id(),
            Self::Stance(stance) => stance.get_item_id(),
            Self::Subsystem(subsystem) => subsystem.get_item_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_item_id(),
        }
    }
    pub(crate) fn get_type_id(&self) -> AItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_type_id(),
            Self::Booster(booster) => booster.get_type_id(),
            Self::Character(character) => character.get_type_id(),
            Self::Charge(charge) => charge.get_type_id(),
            Self::Drone(drone) => drone.get_type_id(),
            Self::Fighter(fighter) => fighter.get_type_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_type_id(),
            Self::Implant(implant) => implant.get_type_id(),
            Self::Module(module) => module.get_type_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_type_id(),
            Self::Rig(rig) => rig.get_type_id(),
            Self::Service(service) => service.get_type_id(),
            Self::Ship(ship) => ship.get_type_id(),
            Self::Skill(skill) => skill.get_type_id(),
            Self::Stance(stance) => stance.get_type_id(),
            Self::Subsystem(subsystem) => subsystem.get_type_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_type_id(),
        }
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_group_id(),
            Self::Booster(booster) => booster.get_group_id(),
            Self::Character(character) => character.get_group_id(),
            Self::Charge(charge) => charge.get_group_id(),
            Self::Drone(drone) => drone.get_group_id(),
            Self::Fighter(fighter) => fighter.get_group_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_group_id(),
            Self::Implant(implant) => implant.get_group_id(),
            Self::Module(module) => module.get_group_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_group_id(),
            Self::Rig(rig) => rig.get_group_id(),
            Self::Service(service) => service.get_group_id(),
            Self::Ship(ship) => ship.get_group_id(),
            Self::Skill(skill) => skill.get_group_id(),
            Self::Stance(stance) => stance.get_group_id(),
            Self::Subsystem(subsystem) => subsystem.get_group_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_group_id(),
        }
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_category_id(),
            Self::Booster(booster) => booster.get_category_id(),
            Self::Character(character) => character.get_category_id(),
            Self::Charge(charge) => charge.get_category_id(),
            Self::Drone(drone) => drone.get_category_id(),
            Self::Fighter(fighter) => fighter.get_category_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_category_id(),
            Self::Implant(implant) => implant.get_category_id(),
            Self::Module(module) => module.get_category_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_category_id(),
            Self::Rig(rig) => rig.get_category_id(),
            Self::Service(service) => service.get_category_id(),
            Self::Ship(ship) => ship.get_category_id(),
            Self::Skill(skill) => skill.get_category_id(),
            Self::Stance(stance) => stance.get_category_id(),
            Self::Subsystem(subsystem) => subsystem.get_category_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_category_id(),
        }
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<RAttrId, AAttrVal>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_attrs(),
            Self::Booster(booster) => booster.get_attrs(),
            Self::Character(character) => character.get_attrs(),
            Self::Charge(charge) => charge.get_attrs(),
            Self::Drone(drone) => drone.get_attrs(),
            Self::Fighter(fighter) => fighter.get_attrs(),
            Self::FwEffect(fw_effect) => fw_effect.get_attrs(),
            Self::Implant(implant) => implant.get_attrs(),
            Self::Module(module) => module.get_attrs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_attrs(),
            Self::Rig(rig) => rig.get_attrs(),
            Self::Service(service) => service.get_attrs(),
            Self::Ship(ship) => ship.get_attrs(),
            Self::Skill(skill) => skill.get_attrs(),
            Self::Stance(stance) => stance.get_attrs(),
            Self::Subsystem(subsystem) => subsystem.get_attrs(),
            Self::SwEffect(sw_effect) => sw_effect.get_attrs(),
        }
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectId, RItemEffectData>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_datas(),
            Self::Booster(booster) => booster.get_effect_datas(),
            Self::Character(character) => character.get_effect_datas(),
            Self::Charge(charge) => charge.get_effect_datas(),
            Self::Drone(drone) => drone.get_effect_datas(),
            Self::Fighter(fighter) => fighter.get_effect_datas(),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_datas(),
            Self::Implant(implant) => implant.get_effect_datas(),
            Self::Module(module) => module.get_effect_datas(),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_datas(),
            Self::Rig(rig) => rig.get_effect_datas(),
            Self::Service(service) => service.get_effect_datas(),
            Self::Ship(ship) => ship.get_effect_datas(),
            Self::Skill(skill) => skill.get_effect_datas(),
            Self::Stance(stance) => stance.get_effect_datas(),
            Self::Subsystem(subsystem) => subsystem.get_effect_datas(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_datas(),
        }
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_defeff_key(),
            Self::Booster(booster) => booster.get_defeff_key(),
            Self::Character(character) => character.get_defeff_key(),
            Self::Charge(charge) => charge.get_defeff_key(),
            Self::Drone(drone) => drone.get_defeff_key(),
            Self::Fighter(fighter) => fighter.get_defeff_key(),
            Self::FwEffect(fw_effect) => fw_effect.get_defeff_key(),
            Self::Implant(implant) => implant.get_defeff_key(),
            Self::Module(module) => module.get_defeff_key(),
            Self::ProjEffect(proj_effect) => proj_effect.get_defeff_key(),
            Self::Rig(rig) => rig.get_defeff_key(),
            Self::Service(service) => service.get_defeff_key(),
            Self::Ship(ship) => ship.get_defeff_key(),
            Self::Skill(skill) => skill.get_defeff_key(),
            Self::Stance(stance) => stance.get_defeff_key(),
            Self::Subsystem(subsystem) => subsystem.get_defeff_key(),
            Self::SwEffect(sw_effect) => sw_effect.get_defeff_key(),
        }
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_skill_reqs(),
            Self::Booster(booster) => booster.get_skill_reqs(),
            Self::Character(character) => character.get_skill_reqs(),
            Self::Charge(charge) => charge.get_skill_reqs(),
            Self::Drone(drone) => drone.get_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_skill_reqs(),
            Self::FwEffect(fw_effect) => fw_effect.get_skill_reqs(),
            Self::Implant(implant) => implant.get_skill_reqs(),
            Self::Module(module) => module.get_skill_reqs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_skill_reqs(),
            Self::Rig(rig) => rig.get_skill_reqs(),
            Self::Service(service) => service.get_skill_reqs(),
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(stance) => stance.get_skill_reqs(),
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(sw_effect) => sw_effect.get_skill_reqs(),
        }
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_axt(),
            Self::Booster(booster) => booster.get_axt(),
            Self::Character(character) => character.get_axt(),
            Self::Charge(charge) => charge.get_axt(),
            Self::Drone(drone) => drone.get_axt(),
            Self::Fighter(fighter) => fighter.get_axt(),
            Self::FwEffect(fw_effect) => fw_effect.get_axt(),
            Self::Implant(implant) => implant.get_axt(),
            Self::Module(module) => module.get_axt(),
            Self::ProjEffect(proj_effect) => proj_effect.get_axt(),
            Self::Rig(rig) => rig.get_axt(),
            Self::Service(service) => service.get_axt(),
            Self::Ship(ship) => ship.get_axt(),
            Self::Skill(skill) => skill.get_axt(),
            Self::Stance(stance) => stance.get_axt(),
            Self::Subsystem(subsystem) => subsystem.get_axt(),
            Self::SwEffect(sw_effect) => sw_effect.get_axt(),
        }
    }
    pub(crate) fn is_ice_harvester(&self) -> bool {
        match self {
            Self::Autocharge(autocharge) => autocharge.is_ice_harvester(),
            Self::Booster(booster) => booster.is_ice_harvester(),
            Self::Character(character) => character.is_ice_harvester(),
            Self::Charge(charge) => charge.is_ice_harvester(),
            Self::Drone(drone) => drone.is_ice_harvester(),
            Self::Fighter(fighter) => fighter.is_ice_harvester(),
            Self::FwEffect(fw_effect) => fw_effect.is_ice_harvester(),
            Self::Implant(implant) => implant.is_ice_harvester(),
            Self::Module(module) => module.is_ice_harvester(),
            Self::ProjEffect(proj_effect) => proj_effect.is_ice_harvester(),
            Self::Rig(rig) => rig.is_ice_harvester(),
            Self::Service(service) => service.is_ice_harvester(),
            Self::Ship(ship) => ship.is_ice_harvester(),
            Self::Skill(skill) => skill.is_ice_harvester(),
            Self::Stance(stance) => stance.is_ice_harvester(),
            Self::Subsystem(subsystem) => subsystem.is_ice_harvester(),
            Self::SwEffect(sw_effect) => sw_effect.is_ice_harvester(),
        }
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_reffs(),
            Self::Booster(booster) => booster.get_reffs(),
            Self::Character(character) => character.get_reffs(),
            Self::Charge(charge) => charge.get_reffs(),
            Self::Drone(drone) => drone.get_reffs(),
            Self::Fighter(fighter) => fighter.get_reffs(),
            Self::FwEffect(fw_effect) => fw_effect.get_reffs(),
            Self::Implant(implant) => implant.get_reffs(),
            Self::Module(module) => module.get_reffs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_reffs(),
            Self::Rig(rig) => rig.get_reffs(),
            Self::Service(service) => service.get_reffs(),
            Self::Ship(ship) => ship.get_reffs(),
            Self::Skill(skill) => skill.get_reffs(),
            Self::Stance(stance) => stance.get_reffs(),
            Self::Subsystem(subsystem) => subsystem.get_reffs(),
            Self::SwEffect(sw_effect) => sw_effect.get_reffs(),
        }
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.update_reffs(reuse_eupdates, src),
            Self::Booster(booster) => booster.update_reffs(reuse_eupdates, src),
            Self::Character(character) => character.update_reffs(reuse_eupdates, src),
            Self::Charge(charge) => charge.update_reffs(reuse_eupdates, src),
            Self::Drone(drone) => drone.update_reffs(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.update_reffs(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.update_reffs(reuse_eupdates, src),
            Self::Implant(implant) => implant.update_reffs(reuse_eupdates, src),
            Self::Module(module) => module.update_reffs(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.update_reffs(reuse_eupdates, src),
            Self::Rig(rig) => rig.update_reffs(reuse_eupdates, src),
            Self::Service(service) => service.update_reffs(reuse_eupdates, src),
            Self::Ship(ship) => ship.update_reffs(reuse_eupdates, src),
            Self::Skill(skill) => skill.update_reffs(reuse_eupdates, src),
            Self::Stance(stance) => stance.update_reffs(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.update_reffs(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.update_reffs(reuse_eupdates, src),
        }
    }
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.stop_all_reffs(reuse_eupdates, src),
            Self::Booster(booster) => booster.stop_all_reffs(reuse_eupdates, src),
            Self::Character(character) => character.stop_all_reffs(reuse_eupdates, src),
            Self::Charge(charge) => charge.stop_all_reffs(reuse_eupdates, src),
            Self::Drone(drone) => drone.stop_all_reffs(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.stop_all_reffs(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.stop_all_reffs(reuse_eupdates, src),
            Self::Implant(implant) => implant.stop_all_reffs(reuse_eupdates, src),
            Self::Module(module) => module.stop_all_reffs(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.stop_all_reffs(reuse_eupdates, src),
            Self::Rig(rig) => rig.stop_all_reffs(reuse_eupdates, src),
            Self::Service(service) => service.stop_all_reffs(reuse_eupdates, src),
            Self::Ship(ship) => ship.stop_all_reffs(reuse_eupdates, src),
            Self::Skill(skill) => skill.stop_all_reffs(reuse_eupdates, src),
            Self::Stance(stance) => stance.stop_all_reffs(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.stop_all_reffs(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.stop_all_reffs(reuse_eupdates, src),
        }
    }
    pub(crate) fn get_effect_key_mode(&self, effect_key: &REffectId) -> EffectMode {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_key_mode(effect_key),
            Self::Booster(booster) => booster.get_effect_key_mode(effect_key),
            Self::Character(character) => character.get_effect_key_mode(effect_key),
            Self::Charge(charge) => charge.get_effect_key_mode(effect_key),
            Self::Drone(drone) => drone.get_effect_key_mode(effect_key),
            Self::Fighter(fighter) => fighter.get_effect_key_mode(effect_key),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_key_mode(effect_key),
            Self::Implant(implant) => implant.get_effect_key_mode(effect_key),
            Self::Module(module) => module.get_effect_key_mode(effect_key),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_key_mode(effect_key),
            Self::Rig(rig) => rig.get_effect_key_mode(effect_key),
            Self::Service(service) => service.get_effect_key_mode(effect_key),
            Self::Ship(ship) => ship.get_effect_key_mode(effect_key),
            Self::Skill(skill) => skill.get_effect_key_mode(effect_key),
            Self::Stance(stance) => stance.get_effect_key_mode(effect_key),
            Self::Subsystem(subsystem) => subsystem.get_effect_key_mode(effect_key),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_key_mode(effect_key),
        }
    }
    pub(crate) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_mode(effect_id, effect_mode, src),
            Self::Booster(booster) => booster.set_effect_mode(effect_id, effect_mode, src),
            Self::Character(character) => character.set_effect_mode(effect_id, effect_mode, src),
            Self::Charge(charge) => charge.set_effect_mode(effect_id, effect_mode, src),
            Self::Drone(drone) => drone.set_effect_mode(effect_id, effect_mode, src),
            Self::Fighter(fighter) => fighter.set_effect_mode(effect_id, effect_mode, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_mode(effect_id, effect_mode, src),
            Self::Implant(implant) => implant.set_effect_mode(effect_id, effect_mode, src),
            Self::Module(module) => module.set_effect_mode(effect_id, effect_mode, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_mode(effect_id, effect_mode, src),
            Self::Rig(rig) => rig.set_effect_mode(effect_id, effect_mode, src),
            Self::Service(service) => service.set_effect_mode(effect_id, effect_mode, src),
            Self::Ship(ship) => ship.set_effect_mode(effect_id, effect_mode, src),
            Self::Skill(skill) => skill.set_effect_mode(effect_id, effect_mode, src),
            Self::Stance(stance) => stance.set_effect_mode(effect_id, effect_mode, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_mode(effect_id, effect_mode, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_mode(effect_id, effect_mode, src),
        }
    }
    pub(crate) fn set_effect_modes(&mut self, effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_modes(effect_modes, src),
            Self::Booster(booster) => booster.set_effect_modes(effect_modes, src),
            Self::Character(character) => character.set_effect_modes(effect_modes, src),
            Self::Charge(charge) => charge.set_effect_modes(effect_modes, src),
            Self::Drone(drone) => drone.set_effect_modes(effect_modes, src),
            Self::Fighter(fighter) => fighter.set_effect_modes(effect_modes, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_modes(effect_modes, src),
            Self::Implant(implant) => implant.set_effect_modes(effect_modes, src),
            Self::Module(module) => module.set_effect_modes(effect_modes, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_modes(effect_modes, src),
            Self::Rig(rig) => rig.set_effect_modes(effect_modes, src),
            Self::Service(service) => service.set_effect_modes(effect_modes, src),
            Self::Ship(ship) => ship.set_effect_modes(effect_modes, src),
            Self::Skill(skill) => skill.set_effect_modes(effect_modes, src),
            Self::Stance(stance) => stance.set_effect_modes(effect_modes, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_modes(effect_modes, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_modes(effect_modes, src),
        }
    }
    pub(crate) fn get_state(&self) -> AState {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_state(),
            Self::Booster(booster) => booster.get_state(),
            Self::Character(character) => character.get_state(),
            Self::Charge(charge) => charge.get_state(),
            Self::Drone(drone) => drone.get_state(),
            Self::Fighter(fighter) => fighter.get_state(),
            Self::FwEffect(fw_effect) => fw_effect.get_state(),
            Self::Implant(implant) => implant.get_state(),
            Self::Module(module) => module.get_state(),
            Self::ProjEffect(proj_effect) => proj_effect.get_state(),
            Self::Rig(rig) => rig.get_state(),
            Self::Service(service) => service.get_state(),
            Self::Ship(ship) => ship.get_state(),
            Self::Skill(skill) => skill.get_state(),
            Self::Stance(stance) => stance.get_state(),
            Self::Subsystem(subsystem) => subsystem.get_state(),
            Self::SwEffect(sw_effect) => sw_effect.get_state(),
        }
    }
    pub(crate) fn src_changed(&mut self, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.src_changed(src),
            Self::Booster(booster) => booster.src_changed(src),
            Self::Character(character) => character.src_changed(src),
            Self::Charge(charge) => charge.src_changed(src),
            Self::Drone(drone) => drone.src_changed(src),
            Self::Fighter(fighter) => fighter.src_changed(src),
            Self::FwEffect(fw_effect) => fw_effect.src_changed(src),
            Self::Implant(implant) => implant.src_changed(src),
            Self::Module(module) => module.src_changed(src),
            Self::ProjEffect(proj_effect) => proj_effect.src_changed(src),
            Self::Rig(rig) => rig.src_changed(src),
            Self::Service(service) => service.src_changed(src),
            Self::Ship(ship) => ship.src_changed(src),
            Self::Skill(skill) => skill.src_changed(src),
            Self::Stance(stance) => stance.src_changed(src),
            Self::Subsystem(subsystem) => subsystem.update_a_data(src),
            Self::SwEffect(sw_effect) => sw_effect.src_changed(src),
        }
    }
    pub(crate) fn is_loaded(&self) -> bool {
        match self {
            Self::Autocharge(autocharge) => autocharge.is_loaded(),
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
            Self::Service(service) => service.is_loaded(),
            Self::Ship(ship) => ship.is_loaded(),
            Self::Skill(skill) => skill.is_loaded(),
            Self::Stance(stance) => stance.is_loaded(),
            Self::Subsystem(subsystem) => subsystem.is_loaded(),
            Self::SwEffect(sw_effect) => sw_effect.is_loaded(),
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Access to item-specific methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn get_fit_key(&self) -> Option<UFitId> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_fit_key()),
            Self::Booster(booster) => Some(booster.get_fit_key()),
            Self::Character(character) => Some(character.get_fit_key()),
            Self::Charge(charge) => Some(charge.get_fit_key()),
            Self::Drone(drone) => Some(drone.get_fit_key()),
            Self::Fighter(fighter) => Some(fighter.get_fit_key()),
            Self::FwEffect(fw_effect) => Some(fw_effect.get_fit_key()),
            Self::Implant(implant) => Some(implant.get_fit_key()),
            Self::Module(module) => Some(module.get_fit_key()),
            Self::ProjEffect(_) => None,
            Self::Rig(rig) => Some(rig.get_fit_key()),
            Self::Service(service) => Some(service.get_fit_key()),
            Self::Ship(ship) => Some(ship.get_fit_key()),
            Self::Skill(skill) => Some(skill.get_fit_key()),
            Self::Stance(stance) => Some(stance.get_fit_key()),
            Self::Subsystem(subsystem) => Some(subsystem.get_fit_key()),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_direct_physics(&self) -> Option<&UPhysics> {
        match self {
            Self::Drone(drone) => Some(drone.get_physics()),
            Self::Fighter(fighter) => Some(fighter.get_physics()),
            Self::Ship(module) => Some(module.get_physics()),
            _ => None,
        }
    }
    pub(crate) fn get_carrier_physics(&self, u_data: &UData) -> UPhysics {
        match self {
            Self::Autocharge(autocharge) => u_data
                .items
                .get(autocharge.get_cont_item_key())
                .get_carrier_physics(u_data),
            Self::Booster(booster) => u_data.get_ship_physics_by_fit_key(booster.get_fit_key()),
            Self::Character(character) => u_data.get_ship_physics_by_fit_key(character.get_fit_key()),
            Self::Charge(charge) => u_data.items.get(charge.get_cont_item_key()).get_carrier_physics(u_data),
            Self::Drone(drone) => *drone.get_physics(),
            Self::Fighter(fighter) => *fighter.get_physics(),
            Self::FwEffect(_) => UPhysics::default(),
            Self::Implant(implant) => u_data.get_ship_physics_by_fit_key(implant.get_fit_key()),
            Self::Module(module) => u_data.get_ship_physics_by_fit_key(module.get_fit_key()),
            Self::ProjEffect(_) => UPhysics::default(),
            Self::Service(service) => u_data.get_ship_physics_by_fit_key(service.get_fit_key()),
            Self::Rig(rig) => u_data.get_ship_physics_by_fit_key(rig.get_fit_key()),
            Self::Ship(ship) => *ship.get_physics(),
            Self::Skill(skill) => u_data.get_ship_physics_by_fit_key(skill.get_fit_key()),
            Self::Stance(stance) => u_data.get_ship_physics_by_fit_key(stance.get_fit_key()),
            Self::Subsystem(subsystem) => u_data.get_ship_physics_by_fit_key(subsystem.get_fit_key()),
            Self::SwEffect(_) => UPhysics::default(),
        }
    }
    pub(crate) fn get_direct_radius(&self) -> AttrVal {
        match self {
            Self::Drone(drone) => drone.get_radius(),
            Self::Fighter(fighter) => fighter.get_radius(),
            Self::Ship(ship) => ship.get_radius(),
            _ => OF(0.0),
        }
    }
    pub(crate) fn get_carrier_radius(&self, u_data: &UData) -> AttrVal {
        match self {
            Self::Autocharge(autocharge) => u_data
                .items
                .get(autocharge.get_cont_item_key())
                .get_carrier_radius(u_data),
            Self::Booster(booster) => u_data.get_ship_radius_by_fit_key(booster.get_fit_key()),
            Self::Character(character) => u_data.get_ship_radius_by_fit_key(character.get_fit_key()),
            Self::Charge(charge) => u_data.items.get(charge.get_cont_item_key()).get_carrier_radius(u_data),
            Self::Drone(drone) => drone.get_radius(),
            Self::Fighter(fighter) => fighter.get_radius(),
            Self::FwEffect(_) => AttrVal::default(),
            Self::Implant(implant) => u_data.get_ship_radius_by_fit_key(implant.get_fit_key()),
            Self::Module(module) => u_data.get_ship_radius_by_fit_key(module.get_fit_key()),
            Self::ProjEffect(_) => AttrVal::default(),
            Self::Service(service) => u_data.get_ship_radius_by_fit_key(service.get_fit_key()),
            Self::Rig(rig) => u_data.get_ship_radius_by_fit_key(rig.get_fit_key()),
            Self::Ship(ship) => ship.get_radius(),
            Self::Skill(skill) => u_data.get_ship_radius_by_fit_key(skill.get_fit_key()),
            Self::Stance(stance) => u_data.get_ship_radius_by_fit_key(stance.get_fit_key()),
            Self::Subsystem(subsystem) => u_data.get_ship_radius_by_fit_key(subsystem.get_fit_key()),
            Self::SwEffect(_) => AttrVal::default(),
        }
    }
    pub(crate) fn get_projs(&self) -> Option<&UProjs> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs()),
            Self::Charge(charge) => Some(charge.get_projs()),
            Self::Drone(drone) => Some(drone.get_projs()),
            Self::Fighter(fighter) => Some(fighter.get_projs()),
            Self::Module(module) => Some(module.get_projs()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs()),
            _ => None,
        }
    }
    pub(crate) fn get_projs_mut(&mut self) -> Option<&mut UProjs> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs_mut()),
            Self::Charge(charge) => Some(charge.get_projs_mut()),
            Self::Drone(drone) => Some(drone.get_projs_mut()),
            Self::Fighter(fighter) => Some(fighter.get_projs_mut()),
            Self::Module(module) => Some(module.get_projs_mut()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs_mut()),
            _ => None,
        }
    }
    pub(crate) fn get_charge_uid(&self) -> Option<UItemId> {
        match self {
            Self::Module(module) => module.get_charge_uid(),
            _ => None,
        }
    }
    pub(crate) fn get_spool(&self) -> Option<Spool> {
        match self {
            Self::Module(module) => module.get_spool(),
            _ => None,
        }
    }
    pub(crate) fn get_reload_optionals(&self) -> Option<bool> {
        match self {
            Self::Module(module) => module.get_reload_optionals(),
            _ => None,
        }
    }
    pub(crate) fn get_rearm_minions(&self) -> Option<bool> {
        match self {
            Self::Fighter(fighter) => fighter.get_rearm_minions(),
            _ => None,
        }
    }
    pub(crate) fn get_autocharges(&self) -> Option<&UAutocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges()),
            _ => None,
        }
    }
    pub(crate) fn get_autocharges_mut(&mut self) -> Option<&mut UAutocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges_mut()),
            _ => None,
        }
    }
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        match self {
            Self::Drone(drone) => drone.get_mutation_data(),
            Self::Module(module) => module.get_mutation_data(),
            _ => None,
        }
    }
    // Methods specific to generic item enum
    pub(crate) fn get_attr(&self, attr_key: RAttrId) -> Option<AAttrVal> {
        match self.get_attrs() {
            Some(attrs) => attrs.get(&attr_key).copied(),
            None => None,
        }
    }
    pub(crate) fn get_effective_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(booster) => booster.get_skill_reqs(),
            Self::Character(_) => None,
            Self::Charge(charge) => charge.get_skill_reqs(),
            Self::Drone(drone) => drone.get_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_skill_reqs(),
            Self::FwEffect(_) => None,
            Self::Implant(implant) => implant.get_skill_reqs(),
            Self::Module(module) => module.get_skill_reqs(),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Service(_) => None,
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn iter_projs(&self) -> Option<impl ExactSizeIterator<Item = (UItemId, Option<UProjData>)>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter()),
            Self::Charge(charge) => Some(charge.get_projs().iter()),
            Self::Drone(drone) => Some(drone.get_projs().iter()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter()),
            Self::Module(module) => Some(module.get_projs().iter()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter()),
            _ => None,
        }
    }
    pub(crate) fn iter_projectees(&self) -> Option<impl ExactSizeIterator<Item = UItemId>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter_projectees()),
            Self::Charge(charge) => Some(charge.get_projs().iter_projectees()),
            Self::Drone(drone) => Some(drone.get_projs().iter_projectees()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter_projectees()),
            Self::Module(module) => Some(module.get_projs().iter_projectees()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter_projectees()),
            _ => None,
        }
    }
    pub(crate) fn iter_charges(&self) -> impl Iterator<Item = UItemId> {
        let charge_key = self.get_charge_uid();
        match self.get_autocharges() {
            Some(autocharges) => Either::Left(charge_key.into_iter().chain(autocharges.values())),
            None => Either::Right(charge_key.into_iter()),
        }
        .into_iter()
    }
}
impl Named for UItem {
    fn get_name() -> &'static str {
        "UItem"
    }
}
impl GetId<ItemId> for UItem {
    fn get_id(&self) -> ItemId {
        self.get_item_id()
    }
}
