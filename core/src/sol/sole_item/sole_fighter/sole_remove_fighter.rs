use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};
use itertools::Itertools;

impl SolarSystem {
    pub fn remove_fighter(&mut self, item_id: &SolItemId) -> Result<(), RemoveFighterError> {
        // Check if everything is correct and collect autocharge IDs to be used later
        let item = self.items.get_item(item_id)?;
        let fighter = item.get_fighter()?;
        let autocharge_ids = fighter.autocharges.values().map(|v| *v).collect_vec();
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove autocharges from services and skeleton
        for autocharge_item_id in autocharge_ids {
            let autocharge_item = self.items.get_item(&autocharge_item_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                autocharge_item,
            );
            self.items.remove_item(&autocharge_item_id);
        }
        // Remove fighter from services
        let item = self.items.get_item(item_id).unwrap();
        let fighter = item.get_fighter().unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Remove fighter from skeleton
        let fit = self.fits.get_fit_mut(&fighter.get_fit_id()).unwrap();
        fit.fighters.remove(item_id);
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFighterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
}
impl From<ItemFoundError> for RemoveFighterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveFighterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
impl std::error::Error for RemoveFighterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFighterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
        }
    }
}
