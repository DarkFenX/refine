use either::Either;

use crate::{
    EffectMode, ItemId, ItemKind, NpcProp, OptionalReload, PValue, RearmMinion, SkillLevel, Spool,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemAttrData, RItemBase, RItemCapConsumer, RState},
    ud::{
        UAutocharge, UBooster, UCharacter, UCharge, UData, UDrone, UFighter, UFitId, UFwEffect, UImplant, UItemId,
        UModule, UPhysics, UProjEffect, URig, UService, UShip, USkill, UStance, USubsystem, USwEffect,
        item::{ItemMutationData, UAutocharges, UEffectUpdates, UProjData, UProjs},
    },
    util::{LibGetId, LibNamed, RMap, RSet},
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
    pub(crate) fn get_item_kind(&self) -> ItemKind {
        match self {
            Self::Autocharge(_) => UAutocharge::get_item_kind(),
            Self::Booster(_) => UBooster::get_item_kind(),
            Self::Character(_) => UCharacter::get_item_kind(),
            Self::Charge(_) => UCharge::get_item_kind(),
            Self::Drone(_) => UDrone::get_item_kind(),
            Self::Fighter(_) => UFighter::get_item_kind(),
            Self::FwEffect(_) => UFwEffect::get_item_kind(),
            Self::Implant(_) => UImplant::get_item_kind(),
            Self::Module(_) => UModule::get_item_kind(),
            Self::ProjEffect(_) => UProjEffect::get_item_kind(),
            Self::Rig(_) => URig::get_item_kind(),
            Self::Service(_) => UService::get_item_kind(),
            Self::Ship(_) => UShip::get_item_kind(),
            Self::Skill(_) => USkill::get_item_kind(),
            Self::Stance(_) => UStance::get_item_kind(),
            Self::Subsystem(_) => USubsystem::get_item_kind(),
            Self::SwEffect(_) => USwEffect::get_item_kind(),
        }
    }
}
impl LibNamed for UItem {
    fn lib_get_name() -> &'static str {
        "UItem"
    }
}
impl LibGetId<ItemId> for UItem {
    fn lib_get_id(&self) -> ItemId {
        self.get_item_id()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Access to base item methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItem {
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
    pub(crate) fn get_type_aid(&self) -> AItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_type_aid(),
            Self::Booster(booster) => booster.get_type_aid(),
            Self::Character(character) => character.get_type_aid(),
            Self::Charge(charge) => charge.get_type_aid(),
            Self::Drone(drone) => drone.get_type_aid(),
            Self::Fighter(fighter) => fighter.get_type_aid(),
            Self::FwEffect(fw_effect) => fw_effect.get_type_aid(),
            Self::Implant(implant) => implant.get_type_aid(),
            Self::Module(module) => module.get_type_aid(),
            Self::ProjEffect(proj_effect) => proj_effect.get_type_aid(),
            Self::Rig(rig) => rig.get_type_aid(),
            Self::Service(service) => service.get_type_aid(),
            Self::Ship(ship) => ship.get_type_aid(),
            Self::Skill(skill) => skill.get_type_aid(),
            Self::Stance(stance) => stance.get_type_aid(),
            Self::Subsystem(subsystem) => subsystem.get_type_aid(),
            Self::SwEffect(sw_effect) => sw_effect.get_type_aid(),
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
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        match self {
            Self::Autocharge(autocharge) => autocharge.update_reffs(reuse_eupdates, r_data),
            Self::Booster(booster) => booster.update_reffs(reuse_eupdates, r_data),
            Self::Character(character) => character.update_reffs(reuse_eupdates, r_data),
            Self::Charge(charge) => charge.update_reffs(reuse_eupdates, r_data),
            Self::Drone(drone) => drone.update_reffs(reuse_eupdates, r_data),
            Self::Fighter(fighter) => fighter.update_reffs(reuse_eupdates, r_data),
            Self::FwEffect(fw_effect) => fw_effect.update_reffs(reuse_eupdates, r_data),
            Self::Implant(implant) => implant.update_reffs(reuse_eupdates, r_data),
            Self::Module(module) => module.update_reffs(reuse_eupdates, r_data),
            Self::ProjEffect(proj_effect) => proj_effect.update_reffs(reuse_eupdates, r_data),
            Self::Rig(rig) => rig.update_reffs(reuse_eupdates, r_data),
            Self::Service(service) => service.update_reffs(reuse_eupdates, r_data),
            Self::Ship(ship) => ship.update_reffs(reuse_eupdates, r_data),
            Self::Skill(skill) => skill.update_reffs(reuse_eupdates, r_data),
            Self::Stance(stance) => stance.update_reffs(reuse_eupdates, r_data),
            Self::Subsystem(subsystem) => subsystem.update_reffs(reuse_eupdates, r_data),
            Self::SwEffect(sw_effect) => sw_effect.update_reffs(reuse_eupdates, r_data),
        }
    }
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        match self {
            Self::Autocharge(autocharge) => autocharge.stop_all_reffs(reuse_eupdates, r_data),
            Self::Booster(booster) => booster.stop_all_reffs(reuse_eupdates, r_data),
            Self::Character(character) => character.stop_all_reffs(reuse_eupdates, r_data),
            Self::Charge(charge) => charge.stop_all_reffs(reuse_eupdates, r_data),
            Self::Drone(drone) => drone.stop_all_reffs(reuse_eupdates, r_data),
            Self::Fighter(fighter) => fighter.stop_all_reffs(reuse_eupdates, r_data),
            Self::FwEffect(fw_effect) => fw_effect.stop_all_reffs(reuse_eupdates, r_data),
            Self::Implant(implant) => implant.stop_all_reffs(reuse_eupdates, r_data),
            Self::Module(module) => module.stop_all_reffs(reuse_eupdates, r_data),
            Self::ProjEffect(proj_effect) => proj_effect.stop_all_reffs(reuse_eupdates, r_data),
            Self::Rig(rig) => rig.stop_all_reffs(reuse_eupdates, r_data),
            Self::Service(service) => service.stop_all_reffs(reuse_eupdates, r_data),
            Self::Ship(ship) => ship.stop_all_reffs(reuse_eupdates, r_data),
            Self::Skill(skill) => skill.stop_all_reffs(reuse_eupdates, r_data),
            Self::Stance(stance) => stance.stop_all_reffs(reuse_eupdates, r_data),
            Self::Subsystem(subsystem) => subsystem.stop_all_reffs(reuse_eupdates, r_data),
            Self::SwEffect(sw_effect) => sw_effect.stop_all_reffs(reuse_eupdates, r_data),
        }
    }
    pub(crate) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_mode(effect_rid),
            Self::Booster(booster) => booster.get_effect_mode(effect_rid),
            Self::Character(character) => character.get_effect_mode(effect_rid),
            Self::Charge(charge) => charge.get_effect_mode(effect_rid),
            Self::Drone(drone) => drone.get_effect_mode(effect_rid),
            Self::Fighter(fighter) => fighter.get_effect_mode(effect_rid),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_mode(effect_rid),
            Self::Implant(implant) => implant.get_effect_mode(effect_rid),
            Self::Module(module) => module.get_effect_mode(effect_rid),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_mode(effect_rid),
            Self::Rig(rig) => rig.get_effect_mode(effect_rid),
            Self::Service(service) => service.get_effect_mode(effect_rid),
            Self::Ship(ship) => ship.get_effect_mode(effect_rid),
            Self::Skill(skill) => skill.get_effect_mode(effect_rid),
            Self::Stance(stance) => stance.get_effect_mode(effect_rid),
            Self::Subsystem(subsystem) => subsystem.get_effect_mode(effect_rid),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_mode(effect_rid),
        }
    }
    pub(crate) fn set_effect_mode(&mut self, effect_aid: AEffectId, effect_mode: EffectMode, r_data: &RData) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Booster(booster) => booster.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Character(character) => character.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Charge(charge) => charge.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Drone(drone) => drone.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Fighter(fighter) => fighter.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Implant(implant) => implant.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Module(module) => module.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Rig(rig) => rig.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Service(service) => service.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Ship(ship) => ship.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Skill(skill) => skill.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Stance(stance) => stance.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::Subsystem(subsystem) => subsystem.set_effect_mode(effect_aid, effect_mode, r_data),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_mode(effect_aid, effect_mode, r_data),
        }
    }
    pub(crate) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_modes(effect_modes, r_data),
            Self::Booster(booster) => booster.set_effect_modes(effect_modes, r_data),
            Self::Character(character) => character.set_effect_modes(effect_modes, r_data),
            Self::Charge(charge) => charge.set_effect_modes(effect_modes, r_data),
            Self::Drone(drone) => drone.set_effect_modes(effect_modes, r_data),
            Self::Fighter(fighter) => fighter.set_effect_modes(effect_modes, r_data),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_modes(effect_modes, r_data),
            Self::Implant(implant) => implant.set_effect_modes(effect_modes, r_data),
            Self::Module(module) => module.set_effect_modes(effect_modes, r_data),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_modes(effect_modes, r_data),
            Self::Rig(rig) => rig.set_effect_modes(effect_modes, r_data),
            Self::Service(service) => service.set_effect_modes(effect_modes, r_data),
            Self::Ship(ship) => ship.set_effect_modes(effect_modes, r_data),
            Self::Skill(skill) => skill.set_effect_modes(effect_modes, r_data),
            Self::Stance(stance) => stance.set_effect_modes(effect_modes, r_data),
            Self::Subsystem(subsystem) => subsystem.set_effect_modes(effect_modes, r_data),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_modes(effect_modes, r_data),
        }
    }
    pub(crate) fn get_state(&self) -> RState {
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
    pub(crate) fn get_r_item_base(&self) -> Option<&RItemBase> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_r_item_base(),
            Self::Booster(booster) => booster.get_r_item_base(),
            Self::Character(character) => character.get_r_item_base(),
            Self::Charge(charge) => charge.get_r_item_base(),
            Self::Drone(drone) => drone.get_r_item_base(),
            Self::Fighter(fighter) => fighter.get_r_item_base(),
            Self::FwEffect(fw_effect) => fw_effect.get_r_item_base(),
            Self::Implant(implant) => implant.get_r_item_base(),
            Self::Module(module) => module.get_r_item_base(),
            Self::ProjEffect(proj_effect) => proj_effect.get_r_item_base(),
            Self::Rig(rig) => rig.get_r_item_base(),
            Self::Service(service) => service.get_r_item_base(),
            Self::Ship(ship) => ship.get_r_item_base(),
            Self::Skill(skill) => skill.get_r_item_base(),
            Self::Stance(stance) => stance.get_r_item_base(),
            Self::Subsystem(subsystem) => subsystem.get_r_item_base(),
            Self::SwEffect(sw_effect) => sw_effect.get_r_item_base(),
        }
    }
    pub(crate) fn get_r_item_attr_data(&self) -> Option<&RItemAttrData> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_r_item_attr_data(),
            Self::Booster(booster) => booster.get_r_item_attr_data(),
            Self::Character(character) => character.get_r_item_attr_data(),
            Self::Charge(charge) => charge.get_r_item_attr_data(),
            Self::Drone(drone) => drone.get_r_item_attr_data(),
            Self::Fighter(fighter) => fighter.get_r_item_attr_data(),
            Self::FwEffect(fw_effect) => fw_effect.get_r_item_attr_data(),
            Self::Implant(implant) => implant.get_r_item_attr_data(),
            Self::Module(module) => module.get_r_item_attr_data(),
            Self::ProjEffect(proj_effect) => proj_effect.get_r_item_attr_data(),
            Self::Rig(rig) => rig.get_r_item_attr_data(),
            Self::Service(service) => service.get_r_item_attr_data(),
            Self::Ship(ship) => ship.get_r_item_attr_data(),
            Self::Skill(skill) => skill.get_r_item_attr_data(),
            Self::Stance(stance) => stance.get_r_item_attr_data(),
            Self::Subsystem(subsystem) => subsystem.get_r_item_attr_data(),
            Self::SwEffect(sw_effect) => sw_effect.get_r_item_attr_data(),
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
    pub(crate) fn r_data_changed(&mut self, r_data: &RData) {
        match self {
            Self::Autocharge(autocharge) => autocharge.r_data_changed(r_data),
            Self::Booster(booster) => booster.r_data_changed(r_data),
            Self::Character(character) => character.r_data_changed(r_data),
            Self::Charge(charge) => charge.r_data_changed(r_data),
            Self::Drone(drone) => drone.r_data_changed(r_data),
            Self::Fighter(fighter) => fighter.r_data_changed(r_data),
            Self::FwEffect(fw_effect) => fw_effect.r_data_changed(r_data),
            Self::Implant(implant) => implant.r_data_changed(r_data),
            Self::Module(module) => module.r_data_changed(r_data),
            Self::ProjEffect(proj_effect) => proj_effect.r_data_changed(r_data),
            Self::Rig(rig) => rig.r_data_changed(r_data),
            Self::Service(service) => service.r_data_changed(r_data),
            Self::Ship(ship) => ship.r_data_changed(r_data),
            Self::Skill(skill) => skill.r_data_changed(r_data),
            Self::Stance(stance) => stance.r_data_changed(r_data),
            Self::Subsystem(subsystem) => subsystem.update_a_data(r_data),
            Self::SwEffect(sw_effect) => sw_effect.r_data_changed(r_data),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Access to item-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItem {
    pub(crate) fn get_fit_uid(&self) -> Option<UFitId> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_fit_uid()),
            Self::Booster(booster) => Some(booster.get_fit_uid()),
            Self::Character(character) => Some(character.get_fit_uid()),
            Self::Charge(charge) => Some(charge.get_fit_uid()),
            Self::Drone(drone) => Some(drone.get_fit_uid()),
            Self::Fighter(fighter) => Some(fighter.get_fit_uid()),
            Self::FwEffect(fw_effect) => Some(fw_effect.get_fit_uid()),
            Self::Implant(implant) => Some(implant.get_fit_uid()),
            Self::Module(module) => Some(module.get_fit_uid()),
            Self::ProjEffect(_) => None,
            Self::Rig(rig) => Some(rig.get_fit_uid()),
            Self::Service(service) => Some(service.get_fit_uid()),
            Self::Ship(ship) => Some(ship.get_fit_uid()),
            Self::Skill(skill) => Some(skill.get_fit_uid()),
            Self::Stance(stance) => Some(stance.get_fit_uid()),
            Self::Subsystem(subsystem) => Some(subsystem.get_fit_uid()),
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
                .get(autocharge.get_cont_item_uid())
                .get_carrier_physics(u_data),
            Self::Booster(booster) => u_data.get_fit_ship_physics(booster.get_fit_uid()),
            Self::Character(character) => u_data.get_fit_ship_physics(character.get_fit_uid()),
            Self::Charge(charge) => u_data.items.get(charge.get_cont_item_uid()).get_carrier_physics(u_data),
            Self::Drone(drone) => *drone.get_physics(),
            Self::Fighter(fighter) => *fighter.get_physics(),
            Self::FwEffect(_) => UPhysics::default(),
            Self::Implant(implant) => u_data.get_fit_ship_physics(implant.get_fit_uid()),
            Self::Module(module) => u_data.get_fit_ship_physics(module.get_fit_uid()),
            Self::ProjEffect(_) => UPhysics::default(),
            Self::Service(service) => u_data.get_fit_ship_physics(service.get_fit_uid()),
            Self::Rig(rig) => u_data.get_fit_ship_physics(rig.get_fit_uid()),
            Self::Ship(ship) => *ship.get_physics(),
            Self::Skill(skill) => u_data.get_fit_ship_physics(skill.get_fit_uid()),
            Self::Stance(stance) => u_data.get_fit_ship_physics(stance.get_fit_uid()),
            Self::Subsystem(subsystem) => u_data.get_fit_ship_physics(subsystem.get_fit_uid()),
            Self::SwEffect(_) => UPhysics::default(),
        }
    }
    pub(crate) fn get_direct_radius(&self) -> PValue {
        match self {
            Self::Drone(drone) => drone.get_radius(),
            Self::Fighter(fighter) => fighter.get_radius(),
            Self::Ship(ship) => ship.get_radius(),
            _ => PValue::default(),
        }
    }
    pub(crate) fn get_carrier_radius(&self, u_data: &UData) -> PValue {
        match self {
            Self::Autocharge(autocharge) => u_data
                .items
                .get(autocharge.get_cont_item_uid())
                .get_carrier_radius(u_data),
            Self::Booster(booster) => u_data.get_fit_ship_radius(booster.get_fit_uid()),
            Self::Character(character) => u_data.get_fit_ship_radius(character.get_fit_uid()),
            Self::Charge(charge) => u_data.items.get(charge.get_cont_item_uid()).get_carrier_radius(u_data),
            Self::Drone(drone) => drone.get_radius(),
            Self::Fighter(fighter) => fighter.get_radius(),
            Self::FwEffect(_) => PValue::default(),
            Self::Implant(implant) => u_data.get_fit_ship_radius(implant.get_fit_uid()),
            Self::Module(module) => u_data.get_fit_ship_radius(module.get_fit_uid()),
            Self::ProjEffect(_) => PValue::default(),
            Self::Service(service) => u_data.get_fit_ship_radius(service.get_fit_uid()),
            Self::Rig(rig) => u_data.get_fit_ship_radius(rig.get_fit_uid()),
            Self::Ship(ship) => ship.get_radius(),
            Self::Skill(skill) => u_data.get_fit_ship_radius(skill.get_fit_uid()),
            Self::Stance(stance) => u_data.get_fit_ship_radius(stance.get_fit_uid()),
            Self::Subsystem(subsystem) => u_data.get_fit_ship_radius(subsystem.get_fit_uid()),
            Self::SwEffect(_) => PValue::default(),
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
    pub(crate) fn get_npc_prop(&self) -> Option<Option<NpcProp>> {
        match self {
            Self::Drone(drone) => Some(drone.get_npc_prop()),
            _ => None,
        }
    }
    pub(crate) fn get_optional_reload(&self) -> Option<OptionalReload> {
        match self {
            Self::Module(module) => module.get_optional_reload(),
            _ => None,
        }
    }
    pub(crate) fn get_rearm_minion(&self) -> Option<RearmMinion> {
        match self {
            Self::Fighter(fighter) => fighter.get_rearm_minion(),
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
    pub(crate) fn get_cap_consumers(&self) -> Option<&Vec<RItemCapConsumer>> {
        match self {
            Self::Module(module) => module.get_r_item_base().map(|v| &v.cap_consumers),
            _ => None,
        }
    }
    // Methods specific to generic item enum
    pub(crate) fn get_effective_skill_reqs(&self) -> Option<&RMap<AItemId, SkillLevel>> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(booster) => booster.get_r_item_base().map(|v| &v.srqs),
            Self::Character(_) => None,
            Self::Charge(charge) => charge.get_r_item_base().map(|v| &v.srqs),
            Self::Drone(drone) => drone.get_r_item_base().map(|v| &v.srqs),
            Self::Fighter(fighter) => fighter.get_r_item_base().map(|v| &v.srqs),
            Self::FwEffect(_) => None,
            Self::Implant(implant) => implant.get_r_item_base().map(|v| &v.srqs),
            Self::Module(module) => module.get_r_item_base().map(|v| &v.srqs),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Service(_) => None,
            Self::Ship(ship) => ship.get_r_item_base().map(|v| &v.srqs),
            Self::Skill(skill) => skill.get_r_item_base().map(|v| &v.srqs),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_r_item_base().map(|v| &v.srqs),
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
        let charge_uid = self.get_charge_uid();
        match self.get_autocharges() {
            Some(autocharges) => Either::Left(charge_uid.into_iter().chain(autocharges.values())),
            None => Either::Right(charge_uid.into_iter()),
        }
        .into_iter()
    }
}
