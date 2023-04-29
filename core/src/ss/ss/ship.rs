use itertools::Itertools;

use crate::{
    ss::item::{Item, Ship},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_ship_id(&self, fit_id: ReeId) -> Option<ReeId> {
        self.items
            .values()
            .find_or_first(|v| match v {
                Item::Ship(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .map(|v| v.get_id())
    }
    pub fn set_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        match self.remove_ship(fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no ship is set
            Err(e) => match e.kind {
                ErrorKind::ItemNotFound => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let ship = Item::Ship(Ship::new(&self.src, item_id, fit_id, type_id));
        self.add_item(ship);
        Ok(item_id)
    }
    pub fn remove_ship(&mut self, fit_id: ReeId) -> Result<()> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Ship(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemNotFound, "ship not found")),
            false => Ok(()),
        }
    }
}
