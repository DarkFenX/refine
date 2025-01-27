use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    err::basic::{ItemKindMatchError, ItemLoadedError},
    sol::uad::item::{
        SolAutocharge, SolAutocharges, SolBooster, SolCharacter, SolCharge, SolDrone, SolEffectModes, SolFighter,
        SolFwEffect, SolImplant, SolItemState, SolModule, SolProjEffect, SolRig, SolShip, SolSkill, SolStance,
        SolSubsystem, SolSwEffect,
    },
    src::Src,
    util::{Named, StMap},
};

#[derive(Clone)]
pub(in crate::sol) enum SolItem {
    Autocharge(SolAutocharge),
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
            Self::Autocharge(_) => SolAutocharge::get_name(),
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
            Self::Autocharge(autocharge) => autocharge.get_id(),
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
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
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
            Self::Ship(ship) => ship.get_type_id(),
            Self::Skill(skill) => skill.get_type_id(),
            Self::Stance(stance) => stance.get_type_id(),
            Self::Subsystem(subsystem) => subsystem.get_type_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_type_id(),
        }
    }
    pub(in crate::sol) fn get_fit_id(&self) -> Option<SolFitId> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_fit_id()),
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
            Self::Autocharge(autocharge) => autocharge.get_effect_modes(),
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
            Self::Autocharge(autocharge) => autocharge.get_effect_modes_mut(),
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
            Self::Autocharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(fighter) => Some(fighter.get_autocharges()),
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
            Self::Autocharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(fighter) => Some(fighter.get_autocharges_mut()),
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
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
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
            Self::Ship(ship) => ship.get_state(),
            Self::Skill(skill) => skill.get_state(),
            Self::Stance(stance) => stance.get_state(),
            Self::Subsystem(subsystem) => subsystem.get_state(),
            Self::SwEffect(sw_effect) => sw_effect.get_state(),
        }
    }
    pub(in crate::sol) fn update_a_data(&mut self, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.update_a_data(src),
            Self::Booster(booster) => booster.update_a_data(src),
            Self::Character(character) => character.update_a_data(src),
            Self::Charge(charge) => charge.update_a_data(src),
            Self::Drone(drone) => drone.update_a_data(src),
            Self::Fighter(fighter) => fighter.update_a_data(src),
            Self::FwEffect(fw_effect) => fw_effect.update_a_data(src),
            Self::Implant(implant) => implant.update_a_data(src),
            Self::Module(module) => module.update_a_data(src),
            Self::ProjEffect(proj_effect) => proj_effect.update_a_data(src),
            Self::Rig(rig) => rig.update_a_data(src),
            Self::Ship(ship) => ship.update_a_data(src),
            Self::Skill(skill) => skill.update_a_data(src),
            Self::Stance(stance) => stance.update_a_data(src),
            Self::Subsystem(subsystem) => subsystem.update_a_data(src),
            Self::SwEffect(sw_effect) => sw_effect.update_a_data(src),
        }
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
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
            Self::Ship(ship) => ship.is_loaded(),
            Self::Skill(skill) => skill.is_loaded(),
            Self::Stance(stance) => stance.is_loaded(),
            Self::Subsystem(subsystem) => subsystem.is_loaded(),
            Self::SwEffect(sw_effect) => sw_effect.is_loaded(),
        }
    }
    pub(in crate::sol) fn can_receive_projs(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
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
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter()),
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.get_projs().iter()),
            Self::Drone(drone) => Some(drone.get_projs().iter()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter()),
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => Some(module.get_projs().iter()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter()),
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    // True if item has any mutation data on it, even if it's not being in effect
    pub(in crate::sol) fn has_mutation_data(&self) -> bool {
        match self {
            Self::Drone(drone) => drone.has_mutation_data(),
            Self::Module(module) => module.has_mutation_data(),
            _ => false,
        }
    }
    pub(in crate::sol) fn iter_projectee_items(&self) -> Option<impl ExactSizeIterator<Item = &SolItemId>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter_items()),
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.get_projs().iter_items()),
            Self::Drone(drone) => Some(drone.get_projs().iter_items()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter_items()),
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => Some(module.get_projs().iter_items()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter_items()),
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    // Extractors of specific items
    pub(in crate::sol) fn get_autocharge(&self) -> Result<&SolAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolAutocharge::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_autocharge_mut(&mut self) -> Result<&mut SolAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolAutocharge::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_booster(&self) -> Result<&SolBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolBooster::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_booster_mut(&mut self) -> Result<&mut SolBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolBooster::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_character(&self) -> Result<&SolCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolCharacter::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_character_mut(&mut self) -> Result<&mut SolCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolCharacter::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_charge(&self) -> Result<&SolCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolCharge::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_charge_mut(&mut self) -> Result<&mut SolCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolCharge::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_drone(&self) -> Result<&SolDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolDrone::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_drone_mut(&mut self) -> Result<&mut SolDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolDrone::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_fighter(&self) -> Result<&SolFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolFighter::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_fighter_mut(&mut self) -> Result<&mut SolFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolFighter::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_implant(&self) -> Result<&SolImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolImplant::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_implant_mut(&mut self) -> Result<&mut SolImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolImplant::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_module(&self) -> Result<&SolModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolModule::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_module_mut(&mut self) -> Result<&mut SolModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolModule::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_rig(&self) -> Result<&SolRig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolRig::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_rig_mut(&mut self) -> Result<&mut SolRig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolRig::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_ship(&self) -> Result<&SolShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolShip::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_ship_mut(&mut self) -> Result<&mut SolShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolShip::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_skill(&self) -> Result<&SolSkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSkill::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_skill_mut(&mut self) -> Result<&mut SolSkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSkill::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_stance(&self) -> Result<&SolStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolStance::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_stance_mut(&mut self) -> Result<&mut SolStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolStance::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_subsystem(&self) -> Result<&SolSubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSubsystem::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_subsystem_mut(&mut self) -> Result<&mut SolSubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSubsystem::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_sw_effect(&self) -> Result<&SolSwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSwEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_sw_effect_mut(&mut self) -> Result<&mut SolSwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolSwEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_fw_effect(&self) -> Result<&SolFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolFwEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_fw_effect_mut(&mut self) -> Result<&mut SolFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolFwEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_proj_effect(&self) -> Result<&SolProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolProjEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    pub(in crate::sol) fn get_proj_effect_mut(&mut self) -> Result<&mut SolProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError::new(
                self.get_id(),
                SolProjEffect::get_name(),
                self.get_name(),
            )),
        }
    }
    // Calculator-specific getters
    // TODO: consider moving to calculator specific item extensions
    pub(in crate::sol) fn get_group_id(&self) -> Option<EItemGrpId> {
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
            Self::Ship(ship) => ship.get_group_id(),
            Self::Skill(skill) => skill.get_group_id(),
            Self::Stance(stance) => stance.get_group_id(),
            Self::Subsystem(subsystem) => subsystem.get_group_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_group_id(),
        }
    }
    pub(in crate::sol) fn get_category_id(&self) -> Option<EItemCatId> {
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
            Self::Ship(ship) => ship.get_category_id(),
            Self::Skill(skill) => skill.get_category_id(),
            Self::Stance(stance) => stance.get_category_id(),
            Self::Subsystem(subsystem) => subsystem.get_category_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_category_id(),
        }
    }
    pub(in crate::sol) fn get_attr(&self, attr_id: &EAttrId) -> Option<AttrVal> {
        match self.get_attrs() {
            Some(attrs) => attrs.get(attr_id).cloned(),
            None => None,
        }
    }

    pub(in crate::sol) fn get_attrs(&self) -> Option<&StMap<EAttrId, AttrVal>> {
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
            Self::Ship(ship) => ship.get_attrs(),
            Self::Skill(skill) => skill.get_attrs(),
            Self::Stance(stance) => stance.get_attrs(),
            Self::Subsystem(subsystem) => subsystem.get_attrs(),
            Self::SwEffect(sw_effect) => sw_effect.get_attrs(),
        }
    }
    pub(in crate::sol) fn get_attrs_err(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        self.get_attrs().ok_or_else(|| ItemLoadedError::new(self.get_id()))
    }
    pub(in crate::sol) fn get_effect_datas(&self) -> Option<&StMap<EEffectId, ad::AItemEffectData>> {
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
            Self::Ship(ship) => ship.get_effect_datas(),
            Self::Skill(skill) => skill.get_effect_datas(),
            Self::Stance(stance) => stance.get_effect_datas(),
            Self::Subsystem(subsystem) => subsystem.get_effect_datas(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_datas(),
        }
    }
    pub(in crate::sol) fn get_effect_datas_err(
        &self,
    ) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.get_effect_datas()
            .ok_or_else(|| ItemLoadedError::new(self.get_id()))
    }
    pub(in crate::sol) fn get_defeff_id(&self) -> Option<Option<EEffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_defeff_id(),
            Self::Booster(booster) => booster.get_defeff_id(),
            Self::Character(character) => character.get_defeff_id(),
            Self::Charge(charge) => charge.get_defeff_id(),
            Self::Drone(drone) => drone.get_defeff_id(),
            Self::Fighter(fighter) => fighter.get_defeff_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_defeff_id(),
            Self::Implant(implant) => implant.get_defeff_id(),
            Self::Module(module) => module.get_defeff_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_defeff_id(),
            Self::Rig(rig) => rig.get_defeff_id(),
            Self::Ship(ship) => ship.get_defeff_id(),
            Self::Skill(skill) => skill.get_defeff_id(),
            Self::Stance(stance) => stance.get_defeff_id(),
            Self::Subsystem(subsystem) => subsystem.get_defeff_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_defeff_id(),
        }
    }
    pub(in crate::sol) fn get_skill_reqs(&self) -> Option<&StMap<EItemId, SkillLevel>> {
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
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(stance) => stance.get_skill_reqs(),
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(sw_effect) => sw_effect.get_skill_reqs(),
        }
    }
    pub(in crate::sol) fn get_effective_skill_reqs(&self) -> Option<&StMap<EItemId, SkillLevel>> {
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
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_extras(),
            Self::Booster(booster) => booster.get_a_extras(),
            Self::Character(character) => character.get_a_extras(),
            Self::Charge(charge) => charge.get_a_extras(),
            Self::Drone(drone) => drone.get_a_extras(),
            Self::Fighter(fighter) => fighter.get_a_extras(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_extras(),
            Self::Implant(implant) => implant.get_a_extras(),
            Self::Module(module) => module.get_a_extras(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_extras(),
            Self::Rig(rig) => rig.get_a_extras(),
            Self::Ship(ship) => ship.get_a_extras(),
            Self::Skill(skill) => skill.get_a_extras(),
            Self::Stance(stance) => stance.get_a_extras(),
            Self::Subsystem(subsystem) => subsystem.get_a_extras(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_extras(),
        }
    }
}
