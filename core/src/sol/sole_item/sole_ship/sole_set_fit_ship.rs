use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::ShipInfo,
        uad::item::{Item, Ship},
    },
};

impl SolarSystem {
    pub fn set_fit_ship(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ShipInfo, SetFitShipError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.set_fit_ship_internal(fit_key, type_id, state);
        Ok(self.get_ship_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn set_fit_ship_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let fit = self.uad.fits.get(fit_key);
        // Remove old ship, if it was set
        if let Some(old_item_key) = fit.ship {
            self.remove_ship_internal(old_item_key).unwrap();
        }
        // Add new ship
        let item_id = self.uad.items.alloc_id();
        let ship = Ship::new(&self.uad.src, item_id, type_id, fit_key, state);
        let ship_kind = ship.get_kind();
        let item = Item::Ship(ship);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.ship = Some(item_key);
        fit.kind = ship_kind;
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitShipError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
