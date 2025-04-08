use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemKey, ItemTypeId, SolarSystem,
        info::ShipInfo,
        uad::item::{Item, Ship},
    },
};

impl SolarSystem {
    pub fn set_fit_ship(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ShipInfo, SetFitShipError> {
        let item_key = self.set_fit_ship_internal(fit_id, type_id, state)?;
        Ok(self.get_ship_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn set_fit_ship_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ItemKey, FitFoundError> {
        let fit = self.uad.fits.get_fit(&fit_id)?;
        // Remove old ship, if it was set
        if let Some(old_item_key) = fit.ship {
            self.remove_ship_internal(old_item_key).unwrap();
        }
        // Add new ship
        let item_id = self.uad.items.alloc_item_id();
        let ship = Ship::new(&self.uad.src, item_id, type_id, fit_id, state);
        let ship_kind = ship.get_kind();
        let item = Item::Ship(ship);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.ship = Some(item_key);
        fit.kind = ship_kind;
        self.add_item_key_to_svc(item_key);
        Ok(item_key)
    }
}

#[derive(Debug)]
pub enum SetFitShipError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitShipError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
