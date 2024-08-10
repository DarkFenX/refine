use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemRemoveError},
    sol::{
        item::{SolAutocharge, SolItem},
        SolarSystem,
    },
    util::Named,
};

impl SolarSystem {
    pub fn remove_item(&mut self, item_id: &SolItemId) -> Result<(), RemoveItemError> {
        let item = self.items.get_item(item_id)?;
        match item {
            // Auto charge can't be removed no matter what
            SolItem::Autocharge(_) => Err(RemoveItemError::UnremovableAutocharge(ItemRemoveError::new(
                *item_id,
                SolAutocharge::get_name(),
            ))),
            // We unwrap when the only reasons of failure are when item is not found and when item
            // kind mismatches, both of which we already checked
            SolItem::Booster(_) => Ok(self.remove_booster(item_id).unwrap()),
            SolItem::Character(_) => Ok(self.remove_character(item_id).unwrap()),
            SolItem::Charge(_) => Ok(self.remove_charge(item_id).unwrap()),
            SolItem::Drone(_) => Ok(self.remove_drone(item_id).unwrap()),
            SolItem::Fighter(_) => Ok(self.remove_fighter(item_id).unwrap()),
            SolItem::FwEffect(_) => Ok(self.remove_fw_effect(item_id).unwrap()),
            SolItem::Implant(_) => Ok(self.remove_implant(item_id).unwrap()),
            SolItem::Module(_) => Ok(self.remove_module(item_id).unwrap()),
            SolItem::ProjEffect(_) => Ok(self.remove_proj_effect(item_id).unwrap()),
            SolItem::Rig(_) => Ok(self.remove_rig(item_id).unwrap()),
            SolItem::Ship(_) => Ok(self.remove_ship(item_id).unwrap()),
            SolItem::Skill(_) => Ok(self.remove_skill(item_id).unwrap()),
            SolItem::Stance(_) => Ok(self.remove_stance(item_id).unwrap()),
            SolItem::Subsystem(_) => Ok(self.remove_subsystem(item_id).unwrap()),
            SolItem::SwEffect(_) => Ok(self.remove_sw_effect(item_id).unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum RemoveItemError {
    ItemNotFound(ItemFoundError),
    UnremovableAutocharge(ItemRemoveError),
}
impl From<ItemFoundError> for RemoveItemError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
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
