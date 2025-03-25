use crate::{
    ad,
    sol::{DmgKinds, FitId, ItemId, svc::calc::CalcAttrVal},
    src::Src,
    util::{StMap, StMapSetL1},
};

use super::shared::RAH_A_EFFECT_ID;

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct RahSim {
    pub(super) resonances: StMap<ItemId, Option<DmgKinds<CalcAttrVal>>>,
    pub(super) by_fit: StMapSetL1<FitId, ItemId>,
    pub(super) cycle_time_a_attr_id: Option<ad::AAttrId>,
    pub(super) sim_running: bool,
}
impl RahSim {
    pub(in crate::sol::svc::calc) fn new(src: &Src) -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
            cycle_time_a_attr_id: src.get_a_effect(&RAH_A_EFFECT_ID).and_then(|v| v.duration_attr_id),
            sim_running: false,
        }
    }
}
