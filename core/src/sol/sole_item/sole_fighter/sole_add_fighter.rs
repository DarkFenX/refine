use itertools::Itertools;

use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolFighter, SolItem, SolItemState},
        item_info::SolFighterInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_fighter(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        state: SolItemState,
    ) -> Result<SolFighterInfo, AddFighterError> {
        // Do everything needed to reserve ID for fighter itself
        let item_id = self.items.alloc_item_id()?;
        let fighter = SolFighter::new(&self.src, item_id, fit_id, type_id, state);
        let item = SolItem::Fighter(fighter);
        self.items.add_item(item);
        // Reserve IDs for autocharges
        if let Err(e) = self.update_item_autocharges(&item_id) {
            // If it failed, remove fighter
            self.items.remove_item(&item_id);
            return Err(e.into());
        }
        // Finalize updates of skeleton
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.fighters.insert(item_id);
        // Add fighter and autocharges to services
        self.add_item_id_to_svcs(&item_id);
        let fighter = self.items.get_item(&item_id).unwrap().get_fighter().unwrap();
        for autocharge_id in fighter.get_autocharges().values().map(|v| *v).collect_vec() {
            self.add_item_id_to_svcs(&autocharge_id);
        }
        // Make info
        let info = self.get_fighter(&item_id).unwrap();
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddFighterError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for AddFighterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFighterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddFighterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddFighterError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
