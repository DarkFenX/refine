use crate::{
    err::basic::ItemKindMatchError,
    ud::{
        UAutocharge, UBooster, UCharacter, UCharge, UDrone, UFighter, UFwEffect, UImplant, UItem, UModule, UProjEffect,
        URig, UService, UShip, USkill, UStance, USubsystem, USwEffect,
    },
    util::LibNamed,
};

impl UItem {
    pub(crate) fn dc_autocharge(&self) -> Result<&UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_autocharge_mut(&mut self) -> Result<&mut UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_booster(&self) -> Result<&UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_booster_mut(&mut self) -> Result<&mut UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_character(&self) -> Result<&UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_character_mut(&mut self) -> Result<&mut UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_charge(&self) -> Result<&UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_charge_mut(&mut self) -> Result<&mut UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_drone(&self) -> Result<&UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_drone_mut(&mut self) -> Result<&mut UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_fighter(&self) -> Result<&UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_fighter_mut(&mut self) -> Result<&mut UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_fw_effect(&self) -> Result<&UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_fw_effect_mut(&mut self) -> Result<&mut UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_implant(&self) -> Result<&UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_implant_mut(&mut self) -> Result<&mut UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_module(&self) -> Result<&UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_module_mut(&mut self) -> Result<&mut UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_proj_effect(&self) -> Result<&UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_proj_effect_mut(&mut self) -> Result<&mut UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_rig(&self) -> Result<&URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_rig_mut(&mut self) -> Result<&mut URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_service(&self) -> Result<&UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_service_mut(&mut self) -> Result<&mut UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_ship(&self) -> Result<&UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_ship_mut(&mut self) -> Result<&mut UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_skill(&self) -> Result<&USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_skill_mut(&mut self) -> Result<&mut USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_stance(&self) -> Result<&UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_stance_mut(&mut self) -> Result<&mut UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_subsystem(&self) -> Result<&USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_subsystem_mut(&mut self) -> Result<&mut USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_sw_effect(&self) -> Result<&USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
    pub(crate) fn dc_sw_effect_mut(&mut self) -> Result<&mut USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::lib_get_name(),
                actual_kind: self.lib_get_name(),
            }),
        }
    }
}
