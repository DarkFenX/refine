use itertools::Itertools;

use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::ShipInfo,
        item::{Item, Ship},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_ship_info(&self, fit_id: &ReeId) -> Result<ShipInfo> {
        self.get_fit_ship(fit_id).map(|v| v.into())
    }
    pub fn set_fit_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ShipInfo> {
        match self.remove_fit_ship(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no ship is set
            Err(e) => match e.kind {
                ErrorKind::ItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let ship = Ship::new(&self.src, item_id, fit_id, type_id);
        let info = ShipInfo::from(&ship);
        let item = Item::Ship(ship);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fit_ship_state(&mut self, fit_id: &ReeId, state: bool) -> Result<()> {
        self.get_fit_ship_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_ship(&mut self, fit_id: &ReeId) -> Result<()> {
        self.check_fit(fit_id)?;
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Ship(s) if s.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemTypeNotFound(Ship::get_name()))),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_ship(&self, fit_id: &ReeId) -> Result<&Ship> {
        self.items
            .values()
            .find_map(|v| match v {
                Item::Ship(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(Ship::get_name())))
    }
    fn get_fit_ship_mut(&mut self, fit_id: &ReeId) -> Result<&mut Ship> {
        self.items
            .values_mut()
            .find_map(|v| match v {
                Item::Ship(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(Ship::get_name())))
    }
}
