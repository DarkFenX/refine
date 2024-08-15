use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolItem, SolShip},
        item_info::SolShipInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_fit_ship(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolShipInfo, SetFitShipError> {
        let fit = self.fits.get_fit(&fit_id)?;
        // Remove old ship, if it was set
        if let Some(old_item_id) = fit.ship {
            self.remove_ship(&old_item_id).unwrap();
        }
        // Add new ship
        // Should be fallible only if we didn't remove old ship, so don't handle failure
        let item_id = self.items.alloc_item_id()?;
        let ship = SolShip::new(&self.src, item_id, fit_id, a_item_id, state);
        let ship_kind = ship.kind;
        let info = SolShipInfo::from(&ship);
        let item = SolItem::Ship(ship);
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        fit.ship = Some(item_id);
        fit.kind = ship_kind;
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum SetFitShipError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for SetFitShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitShipError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for SetFitShipError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
