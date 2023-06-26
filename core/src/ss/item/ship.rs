use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_ship_info(&self, fit_id: &SsFitId) -> Result<ssn::SsShipInfo> {
        self.get_fit_ship(fit_id).map(|v| v.into())
    }
    pub fn set_fit_ship(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsShipInfo> {
        match self.remove_fit_ship(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no ship is set
            Err(e) => match e.kind {
                ErrorKind::SsItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let ship = ssi::SsShip::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsShipInfo::from(&ship);
        let item = ssi::SsItem::Ship(ship);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fit_ship_state(&mut self, fit_id: &SsFitId, state: bool) -> Result<()> {
        self.get_fit_ship_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_ship(&mut self, fit_id: &SsFitId) -> Result<()> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_ship_id(&self, fit_id: &SsFitId) -> Result<SsItemId> {
        self.fits
            .get_fit(fit_id)?
            .ship
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(ssi::SsShip::get_name())))
    }
    fn get_fit_ship(&self, fit_id: &SsFitId) -> Result<&ssi::SsShip> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.items.get_ship(&item_id)
    }
    fn get_fit_ship_mut(&mut self, fit_id: &SsFitId) -> Result<&mut ssi::SsShip> {
        let item_id = self.get_fit_ship_id(fit_id)?;
        self.items.get_ship_mut(&item_id)
    }
}
