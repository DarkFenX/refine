use crate::{
    ad,
    err::basic::{ItemKindMatchError, ItemLoadedError},
    sol::{
        AttrVal, FitId, ItemId,
        uad::item::{
            Autocharge, Autocharges, Booster, Character, Charge, Drone, EffectModes, Fighter, FwEffect, Implant,
            Module, ProjEffect, Rig, Service, Ship, Skill, Stance, Subsystem, SwEffect,
        },
    },
    src::Src,
    util::{HMap, Named},
};

#[derive(Clone)]
pub(in crate::sol) enum Item {
    Autocharge(Autocharge),
    Booster(Booster),
    Character(Character),
    Charge(Charge),
    Drone(Drone),
    Fighter(Fighter),
    FwEffect(FwEffect),
    Implant(Implant),
    Module(Module),
    ProjEffect(ProjEffect),
    Service(Service),
    Rig(Rig),
    Ship(Ship),
    Skill(Skill),
    Stance(Stance),
    Subsystem(Subsystem),
    SwEffect(SwEffect),
}
impl Item {
    pub(in crate::sol) fn get_name(&self) -> &'static str {
        match self {
            Self::Autocharge(_) => Autocharge::get_name(),
            Self::Booster(_) => Booster::get_name(),
            Self::Character(_) => Character::get_name(),
            Self::Charge(_) => Charge::get_name(),
            Self::Drone(_) => Drone::get_name(),
            Self::Fighter(_) => Fighter::get_name(),
            Self::FwEffect(_) => FwEffect::get_name(),
            Self::Implant(_) => Implant::get_name(),
            Self::Module(_) => Module::get_name(),
            Self::ProjEffect(_) => ProjEffect::get_name(),
            Self::Rig(_) => Rig::get_name(),
            Self::Service(_) => Service::get_name(),
            Self::Ship(_) => Ship::get_name(),
            Self::Skill(_) => Skill::get_name(),
            Self::Stance(_) => Stance::get_name(),
            Self::Subsystem(_) => Subsystem::get_name(),
            Self::SwEffect(_) => SwEffect::get_name(),
        }
    }
    pub(in crate::sol) fn get_item_id(&self) -> ItemId {
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
    pub(in crate::sol) fn get_a_item_id(&self) -> ad::AItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_item_id(),
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
            Self::Service(service) => service.get_a_item_id(),
            Self::Ship(ship) => ship.get_a_item_id(),
            Self::Skill(skill) => skill.get_a_item_id(),
            Self::Stance(stance) => stance.get_a_item_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_item_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_item_id(),
        }
    }
    pub(in crate::sol) fn get_fit_id(&self) -> Option<FitId> {
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
            Self::Service(service) => Some(service.get_fit_id()),
            Self::Ship(ship) => Some(ship.get_fit_id()),
            Self::Skill(skill) => Some(skill.get_fit_id()),
            Self::Stance(stance) => Some(stance.get_fit_id()),
            Self::Subsystem(subsystem) => Some(subsystem.get_fit_id()),
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &EffectModes {
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
            Self::Service(service) => service.get_effect_modes(),
            Self::Ship(ship) => ship.get_effect_modes(),
            Self::Skill(skill) => skill.get_effect_modes(),
            Self::Stance(stance) => stance.get_effect_modes(),
            Self::Subsystem(subsystem) => subsystem.get_effect_modes(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_modes(),
        }
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
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
            Self::Service(service) => service.get_effect_modes_mut(),
            Self::Ship(ship) => ship.get_effect_modes_mut(),
            Self::Skill(skill) => skill.get_effect_modes_mut(),
            Self::Stance(stance) => stance.get_effect_modes_mut(),
            Self::Subsystem(subsystem) => subsystem.get_effect_modes_mut(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_modes_mut(),
        }
    }
    pub(in crate::sol) fn get_autocharges(&self) -> Option<&Autocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges()),
            _ => None,
        }
    }
    pub(in crate::sol) fn get_autocharges_mut(&mut self) -> Option<&mut Autocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges_mut()),
            _ => None,
        }
    }
    pub(in crate::sol) fn get_a_state(&self) -> ad::AState {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_state(),
            Self::Booster(booster) => booster.get_a_state(),
            Self::Character(character) => character.get_a_state(),
            Self::Charge(charge) => charge.get_a_state(),
            Self::Drone(drone) => drone.get_a_state(),
            Self::Fighter(fighter) => fighter.get_a_state(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_state(),
            Self::Implant(implant) => implant.get_a_state(),
            Self::Module(module) => module.get_a_state(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_state(),
            Self::Rig(rig) => rig.get_a_state(),
            Self::Service(service) => service.get_a_state(),
            Self::Ship(ship) => ship.get_a_state(),
            Self::Skill(skill) => skill.get_a_state(),
            Self::Stance(stance) => stance.get_a_state(),
            Self::Subsystem(subsystem) => subsystem.get_a_state(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_state(),
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
            Self::Service(service) => service.update_a_data(src),
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
            Self::Service(service) => service.is_loaded(),
            Self::Ship(ship) => ship.is_loaded(),
            Self::Skill(skill) => skill.is_loaded(),
            Self::Stance(stance) => stance.is_loaded(),
            Self::Subsystem(subsystem) => subsystem.is_loaded(),
            Self::SwEffect(sw_effect) => sw_effect.is_loaded(),
        }
    }
    pub(in crate::sol) fn can_receive_projs(&self) -> bool {
        matches!(self, Self::Drone(_) | Self::Fighter(_) | Self::Ship(_))
    }
    pub(in crate::sol) fn iter_projs(&self) -> Option<impl ExactSizeIterator<Item = (&ItemId, &Option<AttrVal>)>> {
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
    pub(in crate::sol) fn iter_projectee_items(&self) -> Option<impl ExactSizeIterator<Item = &ItemId>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter_items()),
            Self::Charge(charge) => Some(charge.get_projs().iter_items()),
            Self::Drone(drone) => Some(drone.get_projs().iter_items()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter_items()),
            Self::Module(module) => Some(module.get_projs().iter_items()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter_items()),
            _ => None,
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
    // Extractors of specific items
    pub(in crate::sol) fn get_autocharge(&self) -> Result<&Autocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Autocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_autocharge_mut(&mut self) -> Result<&mut Autocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Autocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_booster(&self) -> Result<&Booster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Booster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_booster_mut(&mut self) -> Result<&mut Booster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Booster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_character(&self) -> Result<&Character, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Character::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_character_mut(&mut self) -> Result<&mut Character, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Character::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_charge(&self) -> Result<&Charge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Charge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_charge_mut(&mut self) -> Result<&mut Charge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Charge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_drone(&self) -> Result<&Drone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Drone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_drone_mut(&mut self) -> Result<&mut Drone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Drone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_fighter(&self) -> Result<&Fighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Fighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_fighter_mut(&mut self) -> Result<&mut Fighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Fighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_fw_effect(&self) -> Result<&FwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: FwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_fw_effect_mut(&mut self) -> Result<&mut FwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: FwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_implant(&self) -> Result<&Implant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Implant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_implant_mut(&mut self) -> Result<&mut Implant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Implant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_module(&self) -> Result<&Module, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Module::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_module_mut(&mut self) -> Result<&mut Module, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Module::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_proj_effect(&self) -> Result<&ProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_proj_effect_mut(&mut self) -> Result<&mut ProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_rig(&self) -> Result<&Rig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Rig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_rig_mut(&mut self) -> Result<&mut Rig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Rig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_service(&self) -> Result<&Service, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Service::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_service_mut(&mut self) -> Result<&mut Service, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Service::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_ship(&self) -> Result<&Ship, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Ship::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_ship_mut(&mut self) -> Result<&mut Ship, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Ship::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_skill(&self) -> Result<&Skill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Skill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_skill_mut(&mut self) -> Result<&mut Skill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Skill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_stance(&self) -> Result<&Stance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Stance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_stance_mut(&mut self) -> Result<&mut Stance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Stance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_subsystem(&self) -> Result<&Subsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Subsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_subsystem_mut(&mut self) -> Result<&mut Subsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: Subsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_sw_effect(&self) -> Result<&SwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: SwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(in crate::sol) fn get_sw_effect_mut(&mut self) -> Result<&mut SwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: SwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    // Calculator-specific getters
    // TODO: consider moving to calculator specific item extensions
    pub(in crate::sol) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_group_id(),
            Self::Booster(booster) => booster.get_a_group_id(),
            Self::Character(character) => character.get_a_group_id(),
            Self::Charge(charge) => charge.get_a_group_id(),
            Self::Drone(drone) => drone.get_a_group_id(),
            Self::Fighter(fighter) => fighter.get_a_group_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_group_id(),
            Self::Implant(implant) => implant.get_a_group_id(),
            Self::Module(module) => module.get_a_group_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_group_id(),
            Self::Rig(rig) => rig.get_a_group_id(),
            Self::Service(service) => service.get_a_group_id(),
            Self::Ship(ship) => ship.get_a_group_id(),
            Self::Skill(skill) => skill.get_a_group_id(),
            Self::Stance(stance) => stance.get_a_group_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_group_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_group_id(),
        }
    }
    pub(in crate::sol) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_category_id(),
            Self::Booster(booster) => booster.get_a_category_id(),
            Self::Character(character) => character.get_a_category_id(),
            Self::Charge(charge) => charge.get_a_category_id(),
            Self::Drone(drone) => drone.get_a_category_id(),
            Self::Fighter(fighter) => fighter.get_a_category_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_category_id(),
            Self::Implant(implant) => implant.get_a_category_id(),
            Self::Module(module) => module.get_a_category_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_category_id(),
            Self::Rig(rig) => rig.get_a_category_id(),
            Self::Service(service) => service.get_a_category_id(),
            Self::Ship(ship) => ship.get_a_category_id(),
            Self::Skill(skill) => skill.get_a_category_id(),
            Self::Stance(stance) => stance.get_a_category_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_category_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_category_id(),
        }
    }
    pub(in crate::sol) fn get_a_attr(&self, a_attr_id: &ad::AAttrId) -> Option<ad::AAttrVal> {
        match self.get_a_attrs() {
            Some(attrs) => attrs.get(a_attr_id).copied(),
            None => None,
        }
    }

    pub(in crate::sol) fn get_a_attrs(&self) -> Option<&HMap<ad::AAttrId, ad::AAttrVal>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_attrs(),
            Self::Booster(booster) => booster.get_a_attrs(),
            Self::Character(character) => character.get_a_attrs(),
            Self::Charge(charge) => charge.get_a_attrs(),
            Self::Drone(drone) => drone.get_a_attrs(),
            Self::Fighter(fighter) => fighter.get_a_attrs(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_attrs(),
            Self::Implant(implant) => implant.get_a_attrs(),
            Self::Module(module) => module.get_a_attrs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_attrs(),
            Self::Rig(rig) => rig.get_a_attrs(),
            Self::Service(service) => service.get_a_attrs(),
            Self::Ship(ship) => ship.get_a_attrs(),
            Self::Skill(skill) => skill.get_a_attrs(),
            Self::Stance(stance) => stance.get_a_attrs(),
            Self::Subsystem(subsystem) => subsystem.get_a_attrs(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_attrs(),
        }
    }
    pub(in crate::sol) fn get_a_attrs_err(&self) -> Result<&HMap<ad::AAttrId, ad::AAttrVal>, ItemLoadedError> {
        self.get_a_attrs().ok_or_else(|| ItemLoadedError {
            item_id: self.get_item_id(),
        })
    }
    pub(in crate::sol) fn get_a_effect_datas(&self) -> Option<&HMap<ad::AEffectId, ad::AItemEffectData>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_effect_datas(),
            Self::Booster(booster) => booster.get_a_effect_datas(),
            Self::Character(character) => character.get_a_effect_datas(),
            Self::Charge(charge) => charge.get_a_effect_datas(),
            Self::Drone(drone) => drone.get_a_effect_datas(),
            Self::Fighter(fighter) => fighter.get_a_effect_datas(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_effect_datas(),
            Self::Implant(implant) => implant.get_a_effect_datas(),
            Self::Module(module) => module.get_a_effect_datas(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_effect_datas(),
            Self::Rig(rig) => rig.get_a_effect_datas(),
            Self::Service(service) => service.get_a_effect_datas(),
            Self::Ship(ship) => ship.get_a_effect_datas(),
            Self::Skill(skill) => skill.get_a_effect_datas(),
            Self::Stance(stance) => stance.get_a_effect_datas(),
            Self::Subsystem(subsystem) => subsystem.get_a_effect_datas(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_effect_datas(),
        }
    }
    pub(in crate::sol) fn get_a_effect_datas_err(
        &self,
    ) -> Result<&HMap<ad::AEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.get_a_effect_datas().ok_or_else(|| ItemLoadedError {
            item_id: self.get_item_id(),
        })
    }
    pub(in crate::sol) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_defeff_id(),
            Self::Booster(booster) => booster.get_a_defeff_id(),
            Self::Character(character) => character.get_a_defeff_id(),
            Self::Charge(charge) => charge.get_a_defeff_id(),
            Self::Drone(drone) => drone.get_a_defeff_id(),
            Self::Fighter(fighter) => fighter.get_a_defeff_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_defeff_id(),
            Self::Implant(implant) => implant.get_a_defeff_id(),
            Self::Module(module) => module.get_a_defeff_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_defeff_id(),
            Self::Rig(rig) => rig.get_a_defeff_id(),
            Self::Service(service) => service.get_a_defeff_id(),
            Self::Ship(ship) => ship.get_a_defeff_id(),
            Self::Skill(skill) => skill.get_a_defeff_id(),
            Self::Stance(stance) => stance.get_a_defeff_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_defeff_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_defeff_id(),
        }
    }
    pub(in crate::sol) fn get_a_skill_reqs(&self) -> Option<&HMap<ad::AItemId, ad::ASkillLevel>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_skill_reqs(),
            Self::Booster(booster) => booster.get_a_skill_reqs(),
            Self::Character(character) => character.get_a_skill_reqs(),
            Self::Charge(charge) => charge.get_a_skill_reqs(),
            Self::Drone(drone) => drone.get_a_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_a_skill_reqs(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_skill_reqs(),
            Self::Implant(implant) => implant.get_a_skill_reqs(),
            Self::Module(module) => module.get_a_skill_reqs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_skill_reqs(),
            Self::Rig(rig) => rig.get_a_skill_reqs(),
            Self::Service(service) => service.get_a_skill_reqs(),
            Self::Ship(ship) => ship.get_a_skill_reqs(),
            Self::Skill(skill) => skill.get_a_skill_reqs(),
            Self::Stance(stance) => stance.get_a_skill_reqs(),
            Self::Subsystem(subsystem) => subsystem.get_a_skill_reqs(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_skill_reqs(),
        }
    }
    pub(in crate::sol) fn get_effective_a_skill_reqs(&self) -> Option<&HMap<ad::AItemId, ad::ASkillLevel>> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(booster) => booster.get_a_skill_reqs(),
            Self::Character(_) => None,
            Self::Charge(charge) => charge.get_a_skill_reqs(),
            Self::Drone(drone) => drone.get_a_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_a_skill_reqs(),
            Self::FwEffect(_) => None,
            Self::Implant(implant) => implant.get_a_skill_reqs(),
            Self::Module(module) => module.get_a_skill_reqs(),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Service(_) => None,
            Self::Ship(ship) => ship.get_a_skill_reqs(),
            Self::Skill(skill) => skill.get_a_skill_reqs(),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_a_skill_reqs(),
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
            Self::Service(service) => service.get_a_extras(),
            Self::Ship(ship) => ship.get_a_extras(),
            Self::Skill(skill) => skill.get_a_extras(),
            Self::Stance(stance) => stance.get_a_extras(),
            Self::Subsystem(subsystem) => subsystem.get_a_extras(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_extras(),
        }
    }
}
