use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fighter(&mut self, item_id: &ItemId) -> Result<(), RemoveFighterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_fighter_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_fighter_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        // Check if everything is correct and collect autocharge IDs to be used later
        let item = self.uad.items.get(item_key);
        let fighter = item.get_fighter()?;
        let fit_id = fighter.get_fit_id();
        let autocharge_keys = fighter.get_autocharges().values().copied().collect_vec();
        // Remove outgoing projections for fighter and its autocharges
        for &projectee_item_key in fighter.get_projs().iter_projectee_item_keys() {
            let projectee_item = self.uad.items.get(projectee_item_key);
            for &autocharge_key in autocharge_keys.iter() {
                // Update services for autocharge
                self.svc
                    .remove_item_projection(&self.uad, autocharge_key, projectee_item_key, projectee_item);
                // Update user data for autocharge - don't touch data on charge itself, since charge
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            }
            // Update services for fighter
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
            // Update user data for fighter - don't touch data on fighter itself, since fighter will
            // be removed later anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_key);
        // Remove autocharges
        for autocharge_key in autocharge_keys {
            // Update services for autocharge
            self.remove_item_key_from_svc(autocharge_key);
            // Update user data for autocharge - not updating fighter<->autocharge references
            // because both will be removed
            self.uad.items.remove(autocharge_key);
        }
        // Remove fighter
        // Update services for fighter
        self.remove_item_key_from_svc(item_key);
        // Update user data for fighter
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.fighters.remove(&item_key);
        self.uad.items.remove(item_key);
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
