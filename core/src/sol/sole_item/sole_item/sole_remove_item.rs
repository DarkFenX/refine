use crate::{
    err::basic::{ItemFoundError, ItemKindRemoveError},
    sol::{
        ItemId, RmMode, SolarSystem,
        uad::item::{Autocharge, Item},
    },
    util::Named,
};

impl SolarSystem {
    pub fn remove_item(&mut self, item_id: &ItemId, pos_mode: RmMode) -> Result<(), RemoveItemError> {
        let item = self.uad.items.get_item(item_id)?;
        match item {
            // Auto charge can't be removed no matter what
            Item::Autocharge(_) => {
                return Err(RemoveItemError::UnremovableAutocharge(ItemKindRemoveError {
                    item_kind: Autocharge::get_name(),
                }));
            }
            // We unwrap when the only reasons of failure are when item is not found and when item
            // kind mismatches, both of which we already checked
            Item::Booster(_) => self.remove_booster(item_id).unwrap(),
            Item::Character(_) => self.remove_character(item_id).unwrap(),
            Item::Charge(_) => self.remove_charge(item_id).unwrap(),
            Item::Drone(_) => self.remove_drone(item_id).unwrap(),
            Item::Fighter(_) => self.remove_fighter(item_id).unwrap(),
            Item::FwEffect(_) => self.remove_fw_effect(item_id).unwrap(),
            Item::Implant(_) => self.remove_implant(item_id).unwrap(),
            Item::Module(_) => self.remove_module(item_id, pos_mode).unwrap(),
            Item::ProjEffect(_) => self.remove_proj_effect(item_id).unwrap(),
            Item::Rig(_) => self.remove_rig(item_id).unwrap(),
            Item::Service(_) => self.remove_service(item_id).unwrap(),
            Item::Ship(_) => self.remove_ship(item_id).unwrap(),
            Item::Skill(_) => self.remove_skill(item_id).unwrap(),
            Item::Stance(_) => self.remove_stance(item_id).unwrap(),
            Item::Subsystem(_) => self.remove_subsystem(item_id).unwrap(),
            Item::SwEffect(_) => self.remove_sw_effect(item_id).unwrap(),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveItemError {
    ItemNotFound(ItemFoundError),
    UnremovableAutocharge(ItemKindRemoveError),
}
impl std::error::Error for RemoveItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::UnremovableAutocharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::UnremovableAutocharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveItemError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
