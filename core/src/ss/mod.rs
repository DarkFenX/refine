use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
};

pub use svc::SsAttrVal;

use crate::{
    defs::{ReeId, ReeInt},
    src::Src,
    ss::svc::SsSvcs,
    ssi,
    util::Result,
};

mod fit;
mod item;
mod svc;

struct SsView<'a> {
    src: &'a Src,
    items: &'a HashMap<ReeId, ssi::SsItem>,
}
impl<'a> SsView<'a> {
    fn new(src: &'a Src, items: &'a HashMap<ReeId, ssi::SsItem>) -> Self {
        Self { src, items }
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
    svcs: SsSvcs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
            svcs: SsSvcs::new(),
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
        self.svcs
            .get_item_attr_val(&SsView::new(&self.src, &self.items), item_id, attr_id)
    }
    pub fn get_item_attrs(&mut self, item_id: &ReeId) -> Result<HashMap<ReeInt, SsAttrVal>> {
        self.svcs
            .get_item_attr_vals(&SsView::new(&self.src, &self.items), item_id)
    }
}
