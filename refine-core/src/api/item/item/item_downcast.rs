use crate::{
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, Item, ItemCommon, ItemMut, Module, ModuleMut,
        ProjEffect, ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, Skill, SkillMut, Stance, StanceMut,
        Subsystem, SubsystemMut, SwEffect, SwEffectMut,
    },
    err::basic::ItemKindMatchError,
    misc::ItemKind,
};

impl<'s> Item<'s> {
    fn get_item_kind(&self) -> ItemKind {
        match self {
            Self::Autocharge(_) => ItemKind::Autocharge,
            Self::Booster(_) => ItemKind::Booster,
            Self::Character(_) => ItemKind::Character,
            Self::Charge(_) => ItemKind::Charge,
            Self::Drone(_) => ItemKind::Drone,
            Self::Fighter(_) => ItemKind::Fighter,
            Self::FwEffect(_) => ItemKind::FwEffect,
            Self::Implant(_) => ItemKind::Implant,
            Self::Module(_) => ItemKind::Module,
            Self::ProjEffect(_) => ItemKind::ProjEffect,
            Self::Rig(_) => ItemKind::Rig,
            Self::Service(_) => ItemKind::Service,
            Self::Ship(_) => ItemKind::Ship,
            Self::Skill(_) => ItemKind::Skill,
            Self::Stance(_) => ItemKind::Stance,
            Self::Subsystem(_) => ItemKind::Subsystem,
            Self::SwEffect(_) => ItemKind::SwEffect,
        }
    }
    pub fn dc_autocharge(&self) -> Result<&Autocharge<'s>, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Autocharge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_booster(&self) -> Result<&Booster<'s>, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Booster,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_character(&self) -> Result<&Character<'s>, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Character,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_charge(&self) -> Result<&Charge<'s>, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Charge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_drone(&self) -> Result<&Drone<'s>, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Drone,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fighter(&self) -> Result<&Fighter<'s>, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Fighter,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fw_effect(&self) -> Result<&FwEffect<'s>, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::FwEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_implant(&self) -> Result<&Implant<'s>, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Implant,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_module(&self) -> Result<&Module<'s>, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Module,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_proj_effect(&self) -> Result<&ProjEffect<'s>, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::ProjEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_rig(&self) -> Result<&Rig<'s>, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Rig,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_service(&self) -> Result<&Service<'s>, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Service,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_ship(&self) -> Result<&Ship<'s>, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Ship,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_skill(&self) -> Result<&Skill<'s>, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Skill,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_stance(&self) -> Result<&Stance<'s>, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Stance,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_subsystem(&self) -> Result<&Subsystem<'s>, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Subsystem,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_sw_effect(&self) -> Result<&SwEffect<'s>, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::SwEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
}

impl<'s> ItemMut<'s> {
    fn get_item_kind(&self) -> ItemKind {
        match self {
            Self::Autocharge(_) => ItemKind::Autocharge,
            Self::Booster(_) => ItemKind::Booster,
            Self::Character(_) => ItemKind::Character,
            Self::Charge(_) => ItemKind::Charge,
            Self::Drone(_) => ItemKind::Drone,
            Self::Fighter(_) => ItemKind::Fighter,
            Self::FwEffect(_) => ItemKind::FwEffect,
            Self::Implant(_) => ItemKind::Implant,
            Self::Module(_) => ItemKind::Module,
            Self::ProjEffect(_) => ItemKind::ProjEffect,
            Self::Rig(_) => ItemKind::Rig,
            Self::Service(_) => ItemKind::Service,
            Self::Ship(_) => ItemKind::Ship,
            Self::Skill(_) => ItemKind::Skill,
            Self::Stance(_) => ItemKind::Stance,
            Self::Subsystem(_) => ItemKind::Subsystem,
            Self::SwEffect(_) => ItemKind::SwEffect,
        }
    }
    pub fn dc_autocharge(&mut self) -> Result<&mut AutochargeMut<'s>, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Autocharge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_booster(&mut self) -> Result<&mut BoosterMut<'s>, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Booster,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_character(&mut self) -> Result<&mut CharacterMut<'s>, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Character,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_charge(&mut self) -> Result<&mut ChargeMut<'s>, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Charge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_drone(&mut self) -> Result<&mut DroneMut<'s>, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Drone,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fighter(&mut self) -> Result<&mut FighterMut<'s>, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Fighter,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fw_effect(&mut self) -> Result<&mut FwEffectMut<'s>, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::FwEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_implant(&mut self) -> Result<&mut ImplantMut<'s>, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Implant,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_module(&mut self) -> Result<&mut ModuleMut<'s>, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Module,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_proj_effect(&mut self) -> Result<&mut ProjEffectMut<'s>, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::ProjEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_rig(&mut self) -> Result<&mut RigMut<'s>, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Rig,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_service(&mut self) -> Result<&mut ServiceMut<'s>, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Service,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_ship(&mut self) -> Result<&mut ShipMut<'s>, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Ship,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_skill(&mut self) -> Result<&mut SkillMut<'s>, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Skill,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_stance(&mut self) -> Result<&mut StanceMut<'s>, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Stance,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_subsystem(&mut self) -> Result<&mut SubsystemMut<'s>, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Subsystem,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_sw_effect(&mut self) -> Result<&mut SwEffectMut<'s>, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::SwEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
}
