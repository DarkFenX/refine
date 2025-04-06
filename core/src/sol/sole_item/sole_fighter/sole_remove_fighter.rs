use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fighter(&mut self, item_id: &ItemId) -> Result<(), RemoveFighterError> {
        // Check if everything is correct and collect autocharge IDs to be used later
        let item = self.uad.items.get_by_id(item_id)?;
        let fighter = item.get_fighter()?;
        let fit_id = fighter.get_fit_id();
        let autocharge_ids = fighter.get_autocharges().values().copied().collect_vec();
        // Remove outgoing projections for fighter and its autocharges
        for projectee_item_id in fighter.get_projs().iter_items() {
            let projectee_item = self.uad.items.get_by_id(projectee_item_id).unwrap();
            for autocharge_id in autocharge_ids.iter() {
                // Update services for autocharge
                let autocharge_item = self.uad.items.get_by_id(autocharge_id).unwrap();
                self.svc
                    .remove_item_projection(&self.uad, autocharge_item, projectee_item);
                // Update user data for autocharge - don't touch data on charge itself, since charge
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(autocharge_id, projectee_item_id);
            }
            // Update services for fighter
            self.svc.remove_item_projection(&self.uad, item, projectee_item);
            // Update user data for fighter - don't touch data on fighter itself, since fighter will
            // be removed later anyway
            self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove autocharges
        for autocharge_id in autocharge_ids {
            // Update services for autocharge
            self.remove_item_id_from_svc(&autocharge_id);
            // Update user data for autocharge - not updating fighter<->autocharge references
            // because both will be removed
            self.uad.items.remove_by_id(&autocharge_id);
        }
        // Remove fighter
        // Update services for fighter
        self.remove_item_id_from_svc(item_id);
        // Update user data for fighter
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.fighters.remove(item_id);
        self.uad.items.remove_by_id(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFighterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
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
