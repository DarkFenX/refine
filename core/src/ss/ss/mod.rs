use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
    sync::Arc,
};

use crate::{
    defs::{ReeId, ReeInt},
    src::Src,
    ss::{
        calc::{AttrVal, CalcSvc},
        helpers,
    },
    ssi, ssn,
    util::{Error, ErrorKind, Result},
};

mod booster;
mod character;
mod charge;
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

pub(in crate::ss) struct SsInnerData<'a> {
    pub(in crate::ss) src: &'a Arc<Src>,
    pub(in crate::ss) items: &'a HashMap<ReeId, ssi::Item>,
    pub(in crate::ss) calc: &'a mut CalcSvc,
}
impl<'a> SsInnerData<'a> {
    fn new(src: &'a Arc<Src>, items: &'a HashMap<ReeId, ssi::Item>, calc: &'a mut CalcSvc) -> Self {
        Self { src, items, calc }
    }
}

pub struct SolarSystem {
    src: Arc<Src>,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, ssi::Item>,
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
                return Err(Error::new(ErrorKind::ItemIdAllocFailed));
            }
        }
        Ok(self.item_cnt.0)
    }
    fn add_item(&mut self, item: ssi::Item) {
        let item_id = item.get_id();
        self.items.insert(item_id, item);
        let item = self.items.get(&item_id).unwrap();
        helpers::add_item(item, &mut SsInnerData::new(&self.src, &self.items, &mut self.calc));
    }
    fn get_item(&self, item_id: &ReeId) -> Result<&ssi::Item> {
        self.items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    fn get_item_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::Item> {
        self.items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub fn get_item_info(&self, item_id: &ReeId) -> Result<ssn::ItemInfo> {
        self.get_item(item_id).map(|v| ssn::ItemInfo::from_item(v, self))
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> Result<()> {
        let main = match self.items.get(item_id) {
            Some(item) => item,
            None => return Err(Error::new(ErrorKind::ItemIdNotFound(*item_id))),
        };
        helpers::remove_item(&main, &mut SsInnerData::new(&self.src, &self.items, &mut self.calc));
        match main {
            // Remove reference to charge if it's charge which we're removing
            // Item::Charge(c) => match self.items.get_mut(&c.cont_id) {
            //     None => {
            //         self.items.remove(item_id);
            //         return Ok(())
            //     },
            //     Some(other) => match other {
            //         Item::Module(m) => m.charge_id = None,
            //         _ => (),
            //     },
            // },
            // Remove charge if we're removing a module, charges cannot exist without their carrier
            ssi::Item::Module(m) => match m.charge_id {
                Some(other_id) => match self.items.remove(&other_id) {
                    Some(charge) => {
                        helpers::remove_item(&charge, &mut SsInnerData::new(&self.src, &self.items, &mut self.calc))
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        };
        self.items.remove(item_id);
        Ok(())
    }
    // Attribute calculator
    pub fn get_item_attr(&mut self, item_id: &ReeId, attr_id: &ReeInt) -> Result<AttrVal> {
        self.calc.get_item_attr_val(item_id, attr_id, &self.src, &self.items)
    }
    pub fn get_item_attrs(&mut self, item_id: &ReeId) -> Result<HashMap<ReeInt, AttrVal>> {
        self.calc.get_item_attr_vals(item_id, &self.src, &self.items)
    }
}
