use crate::{
    misc::DmgKinds,
    svc::calc::CalcAttrVals,
    ud::{UFitKey, UItemKey},
    util::{RMap, RMapRSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct RahSim {
    pub(super) resonances: RMap<UItemKey, Option<DmgKinds<CalcAttrVals>>>,
    pub(super) by_fit: RMapRSet<UFitKey, UItemKey>,
    pub(super) sim_running: bool,
}
impl RahSim {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            resonances: RMap::new(),
            by_fit: RMapRSet::new(),
            sim_running: false,
        }
    }
}
