use crate::{
    Autocharge, Booster, Character, Charge, Drone, Fighter, FwEffect, Implant, Item, ItemCommon, ItemKind, Module,
    ProjEffect, Rig, Service, Ship, Skill, Stance, Subsystem, SwEffect, err::ItemKindMatchError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    pub fn dc_autocharge(&mut self) -> Result<&mut Autocharge<'s>, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Autocharge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_booster(&mut self) -> Result<&mut Booster<'s>, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Booster,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_character(&mut self) -> Result<&mut Character<'s>, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Character,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_charge(&mut self) -> Result<&mut Charge<'s>, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Charge,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_drone(&mut self) -> Result<&mut Drone<'s>, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Drone,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fighter(&mut self) -> Result<&mut Fighter<'s>, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Fighter,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_fw_effect(&mut self) -> Result<&mut FwEffect<'s>, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::FwEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_implant(&mut self) -> Result<&mut Implant<'s>, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Implant,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_module(&mut self) -> Result<&mut Module<'s>, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Module,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_proj_effect(&mut self) -> Result<&mut ProjEffect<'s>, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::ProjEffect,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_rig(&mut self) -> Result<&mut Rig<'s>, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Rig,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_service(&mut self) -> Result<&mut Service<'s>, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Service,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_ship(&mut self) -> Result<&mut Ship<'s>, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Ship,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_skill(&mut self) -> Result<&mut Skill<'s>, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Skill,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_stance(&mut self) -> Result<&mut Stance<'s>, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Stance,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_subsystem(&mut self) -> Result<&mut Subsystem<'s>, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: ItemKind::Subsystem,
                actual_kind: self.get_item_kind(),
            }),
        }
    }
    pub fn dc_sw_effect(&mut self) -> Result<&mut SwEffect<'s>, ItemKindMatchError> {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    fn get_item_kind(&self) -> ItemKind {
        match self {
            Self::Autocharge(..) => ItemKind::Autocharge,
            Self::Booster(..) => ItemKind::Booster,
            Self::Character(..) => ItemKind::Character,
            Self::Charge(..) => ItemKind::Charge,
            Self::Drone(..) => ItemKind::Drone,
            Self::Fighter(..) => ItemKind::Fighter,
            Self::FwEffect(..) => ItemKind::FwEffect,
            Self::Implant(..) => ItemKind::Implant,
            Self::Module(..) => ItemKind::Module,
            Self::ProjEffect(..) => ItemKind::ProjEffect,
            Self::Rig(..) => ItemKind::Rig,
            Self::Service(..) => ItemKind::Service,
            Self::Ship(..) => ItemKind::Ship,
            Self::Skill(..) => ItemKind::Skill,
            Self::Stance(..) => ItemKind::Stance,
            Self::Subsystem(..) => ItemKind::Subsystem,
            Self::SwEffect(..) => ItemKind::SwEffect,
        }
    }
}
