use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolShip},
        item_info::SolShipInfo,
        SolView, SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_ship_info(&self, fit_id: &SolFitId) -> Result<SolShipInfo> {
        self.get_fit_ship(fit_id).map(|v| v.into())
    }
    pub fn set_fit_ship(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolShipInfo> {
        match self.remove_fit_ship(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no ship is set
            Err(e) => match e.kind {
                ErrorKind::SolItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let ship = SolShip::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolShipInfo::from(&ship);
        let item = SolItem::Ship(ship);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_ship_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let ship = self.items.get_ship_mut(item_id)?;
        let old_state = ship.state;
        ship.set_bool_state(state);
        let new_state = ship.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
    pub fn set_fit_ship_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<()> {
        let ship = self.get_fit_ship_id(fit_id)?;
        self.set_ship_state(&ship, state)
    }
    pub fn remove_fit_ship(&mut self, fit_id: &SolFitId) -> Result<()> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_ship_id(&self, fit_id: &SolFitId) -> Result<SolItemId> {
        self.fits
            .get_fit(fit_id)?
            .ship
            .ok_or_else(|| Error::new(ErrorKind::SolItemTypeNotFound(SolShip::get_name())))
    }
    fn get_fit_ship(&self, fit_id: &SolFitId) -> Result<&SolShip> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.items.get_ship(&item_id)
    }
    fn get_fit_ship_mut(&mut self, fit_id: &SolFitId) -> Result<&mut SolShip> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.items.get_ship_mut(&item_id)
    }
}
