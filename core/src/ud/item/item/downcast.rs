use crate::{
    err::basic::ItemKindMatchError,
    ud::{
        UAutocharge, UBooster, UCharacter, UCharge, UDrone, UFighter, UFwEffect, UImplant, UItem, UModule, UProjEffect,
        URig, UService, UShip, USkill, UStance, USubsystem, USwEffect,
    },
    util::Named,
};

impl UItem {
    pub(crate) fn get_autocharge(&self) -> Result<&UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_autocharge_mut(&mut self) -> Result<&mut UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster(&self) -> Result<&UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster_mut(&mut self) -> Result<&mut UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character(&self) -> Result<&UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character_mut(&mut self) -> Result<&mut UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge(&self) -> Result<&UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge_mut(&mut self) -> Result<&mut UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone(&self) -> Result<&UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone_mut(&mut self) -> Result<&mut UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter(&self) -> Result<&UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter_mut(&mut self) -> Result<&mut UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect(&self) -> Result<&UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect_mut(&mut self) -> Result<&mut UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant(&self) -> Result<&UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant_mut(&mut self) -> Result<&mut UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module(&self) -> Result<&UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module_mut(&mut self) -> Result<&mut UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect(&self) -> Result<&UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect_mut(&mut self) -> Result<&mut UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig(&self) -> Result<&URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig_mut(&mut self) -> Result<&mut URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service(&self) -> Result<&UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service_mut(&mut self) -> Result<&mut UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship(&self) -> Result<&UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship_mut(&mut self) -> Result<&mut UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill(&self) -> Result<&USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill_mut(&mut self) -> Result<&mut USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance(&self) -> Result<&UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance_mut(&mut self) -> Result<&mut UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem(&self) -> Result<&USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem_mut(&mut self) -> Result<&mut USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect(&self) -> Result<&USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect_mut(&mut self) -> Result<&mut USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
}
