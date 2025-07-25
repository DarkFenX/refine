use super::shared::RAH_EFFECT_ID;
use crate::{
    ad,
    misc::DmgKinds,
    src::Src,
    svc::calc::CalcAttrVal,
    ud::{UFitKey, UItemKey},
    util::{RMap, RMapRSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct RahSim {
    pub(super) resonances: RMap<UItemKey, Option<DmgKinds<CalcAttrVal>>>,
    pub(super) by_fit: RMapRSet<UFitKey, UItemKey>,
    pub(super) cycle_time_a_attr_id: Option<ad::AAttrId>,
    pub(super) sim_running: bool,
}
impl RahSim {
    pub(in crate::svc::calc) fn new(src: &Src) -> Self {
        Self {
            resonances: RMap::new(),
            by_fit: RMapRSet::new(),
            cycle_time_a_attr_id: src.get_r_effect(&RAH_EFFECT_ID).and_then(|v| v.get_duration_attr_id()),
            sim_running: false,
        }
    }
}
