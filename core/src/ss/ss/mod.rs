use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    src::Src,
    ss::{calc::CalcSvc, helpers, item::Item},
    Error, ErrorKind, ReeFloat, ReeId, ReeIdx, ReeInt, Result,
};

mod booster;
mod character;
mod drone;
mod fighter;
mod fit;
mod implant;
mod module;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;
mod sw_effect;

pub struct SolarSystem {
    src: Arc<Src>,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, Item>,
    calc: CalcSvc,
}
impl SolarSystem {
    pub fn new(src: Arc<Src>) -> Self {
        Self {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
            calc: CalcSvc::new(),
        }
    }
    pub fn set_src(&mut self, src: Arc<Src>) {
        for item in self.items.values_mut() {
            item.reload_cached_item(&src)
        }
        self.src = src;
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
    fn add_item(&mut self, item: Item) {
        helpers::add_item(&item, &self.src, &mut self.calc);
        self.items.insert(item.get_id(), item);
    }
    fn get_positions(&self, item_ids: &Vec<ReeId>) -> Vec<ReeIdx> {
        item_ids
            .iter()
            .filter_map(|v| self.items.get(v))
            .filter_map(|v| v.get_pos())
            .collect_vec()
    }
    fn get_item(&self, item_id: &ReeId) -> Result<&Item> {
        self.items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))
    }
    fn get_item_mut(&mut self, item_id: &ReeId) -> Result<&mut Item> {
        self.items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> bool {
        match self.items.remove(item_id) {
            None => false,
            Some(main) => {
                helpers::remove_item(&main, &self.src, &mut self.calc);
                match main {
                    // Remove reference to charge if it's charge which we're removing
                    Item::Charge(c) => match self.items.get_mut(&c.cont) {
                        None => return true,
                        Some(other) => match other {
                            Item::ModuleHigh(m) => m.charge = None,
                            Item::ModuleMid(m) => m.charge = None,
                            Item::ModuleLow(m) => m.charge = None,
                            _ => (),
                        },
                    },
                    // Remove charge if we're removing a module, charges cannot exist without their carrier
                    Item::ModuleHigh(m) => match m.charge {
                        Some(other_id) => match self.items.remove(&other_id) {
                            Some(charge) => helpers::remove_item(&charge, &self.src, &mut self.calc),
                            _ => (),
                        },
                        _ => (),
                    },
                    Item::ModuleMid(m) => match m.charge {
                        Some(other_id) => match self.items.remove(&other_id) {
                            Some(charge) => helpers::remove_item(&charge, &self.src, &mut self.calc),
                            _ => (),
                        },
                        None => (),
                    },
                    Item::ModuleLow(m) => match m.charge {
                        Some(other_id) => match self.items.remove(&other_id) {
                            Some(charge) => helpers::remove_item(&charge, &self.src, &mut self.calc),
                            _ => (),
                        },
                        None => (),
                    },
                    _ => (),
                };
                true
            }
        }
    }
    // Attribute calculator
    // TODO: refactor this and child functions into Result<>
    pub fn get_item_attr(&mut self, item_id: &ReeId, attr_id: &ReeInt) -> Option<ReeFloat> {
        self.calc.get_attr_val(item_id, attr_id, &self.src, &self.items)
    }
    pub fn get_item_attrs(&mut self, item_id: &ReeId) -> Option<HashMap<ReeInt, ReeFloat>> {
        self.calc.get_attr_vals(item_id, &self.src, &self.items)
    }
}
