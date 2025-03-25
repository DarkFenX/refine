use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
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
        let fit = self.uad.fits.get_fit(&fit_id)?;
        // Remove old ship, if it was set
        if let Some(old_item_id) = fit.ship {
            self.remove_ship(&old_item_id).unwrap();
        }
        // Add new ship
        let item_id = self.uad.items.alloc_item_id();
        let ship = Ship::new(&self.uad.src, item_id, type_id, fit_id, state);
        let ship_kind = ship.get_kind();
        let info = ShipInfo::from(&ship);
        let item = Item::Ship(ship);
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.ship = Some(item_id);
        fit.kind = ship_kind;
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
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
