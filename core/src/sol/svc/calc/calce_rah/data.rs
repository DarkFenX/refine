use crate::{
    ad,
    sol::{DmgKinds, FitId, ItemKey, svc::calc::CalcAttrVal},
    src::Src,
    util::{RMap, RMapRSet},
};

use super::shared::RAH_EFFECT_ID;

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct RahSim {
    pub(super) resonances: RMap<ItemKey, Option<DmgKinds<CalcAttrVal>>>,
    pub(super) by_fit: RMapRSet<FitId, ItemKey>,
    pub(super) cycle_time_a_attr_id: Option<ad::AAttrId>,
    pub(super) sim_running: bool,
}
impl RahSim {
    pub(in crate::sol::svc::calc) fn new(src: &Src) -> Self {
        Self {
            resonances: RMap::new(),
            by_fit: RMapRSet::new(),
            cycle_time_a_attr_id: src.get_a_effect(&RAH_EFFECT_ID).and_then(|v| v.duration_attr_id),
            sim_running: false,
        }
    }
}
