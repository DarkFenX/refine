use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    Error,
    ErrorKind,
    item::{Item, Ship}, ReeId, ReeInt, Result, src::{Src, SrcMgr},
};

pub struct SolarSystem {
    src: Arc<Src>,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, Item>,
}
impl SolarSystem {
    pub fn new(src: Arc<Src>) -> SolarSystem {
        SolarSystem {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
        }
    }
    // Fit methods
    pub fn add_fit(&mut self) -> Result<ReeId> {
        let id = self.alloc_fit_id()?;
        self.fits.insert(id);
        Ok(id)
    }
    pub fn remove_fit(&mut self, fit_id: ReeId) -> bool {
        self.items.drain_filter(|_, v| v.get_fit_id() == fit_id);
        self.fits.remove(&fit_id)
    }
    // Ship methods
    pub fn set_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_ship(fit_id)?;
        let ship_id = self.alloc_item_id()?;
        let ship = Item::Ship(Ship::new(ship_id, fit_id, type_id));
        self.items.insert(ship_id, ship);
        Ok(ship_id)
    }
    pub fn remove_ship(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| matches!(v, Item::Ship(_)) && v.get_fit_id() == fit_id)
            .collect_vec();
        Ok(!removed.is_empty())
    }
    // ID allocation
    fn alloc_fit_id(&mut self) -> Result<ReeId> {
        let start = self.fit_cnt;
        while self.fits.contains(&self.fit_cnt.0) {
            self.fit_cnt += 1;
            if start == self.fit_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate fit ID"));
            }
        }
        Ok(self.fit_cnt.0)
    }
    fn alloc_item_id(&mut self) -> Result<ReeId> {
        let start = self.item_cnt;
        while self.items.contains_key(&self.item_cnt.0) {
            self.item_cnt += 1;
            if start == self.item_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate item ID"));
            }
        }
        Ok(self.item_cnt.0)
    }
}
