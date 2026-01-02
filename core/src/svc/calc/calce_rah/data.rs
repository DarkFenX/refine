use crate::{
    misc::DmgKinds,
    svc::calc::CalcAttrVals,
    ud::{UFitId, UItemId},
    util::{RMap, RMapRSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct RahSim {
    pub(super) resonances: RMap<UItemId, Option<DmgKinds<CalcAttrVals>>>,
    pub(super) by_fit: RMapRSet<UFitId, UItemId>,
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
