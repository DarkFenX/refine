use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
};

use crate::{
    defs::{ReeId, ReeInt},
    src::Src,
    ss::calc::{CalcSvc, SsAttrVal},
    ssi,
    util::Result,
};

mod fit;
mod item;

pub(in crate::ss) struct SsInnerData<'a> {
    pub(in crate::ss) src: &'a Src,
    pub(in crate::ss) items: &'a HashMap<ReeId, ssi::SsItem>,
    pub(in crate::ss) calc: &'a mut CalcSvc,
}
impl<'a> SsInnerData<'a> {
    fn new(src: &'a Src, items: &'a HashMap<ReeId, ssi::SsItem>, calc: &'a mut CalcSvc) -> Self {
        Self { src, items, calc }
    }
}

pub struct SolarSystem {
    src: Src,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, ssi::SsItem>,
    calc: CalcSvc,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
            calc: CalcSvc::new(),
        }
    }
    pub fn set_src(&mut self, src: Src) {
        for item in self.items.values_mut() {
            item.reload_a_item(&src)
        }
        self.src = src;
    }
    // Attribute calculator
    pub fn get_item_attr(&mut self, item_id: &ReeId, attr_id: &ReeInt) -> Result<SsAttrVal> {
        self.calc.get_item_attr_val(item_id, attr_id, &self.src, &self.items)
    }
    pub fn get_item_attrs(&mut self, item_id: &ReeId) -> Result<HashMap<ReeInt, SsAttrVal>> {
        self.calc.get_item_attr_vals(item_id, &self.src, &self.items)
    }
}
