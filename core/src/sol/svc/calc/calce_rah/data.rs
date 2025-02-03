use crate::{
    defs::{EAttrId, SolFitId, SolItemId},
    sol::{svc::calc::SolAttrVal, SolDmgKinds},
    src::Src,
    util::{StMap, StMapSetL1},
};

use super::shared::RAH_EFFECT_ID;

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolRahSim {
    pub(super) resonances: StMap<SolItemId, Option<SolDmgKinds<SolAttrVal>>>,
    pub(super) by_fit: StMapSetL1<SolFitId, SolItemId>,
    pub(super) cycle_time_attr_id: Option<EAttrId>,
    pub(super) sim_running: bool,
}
impl SolRahSim {
    pub(in crate::sol::svc::calc) fn new(src: &Src) -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
            cycle_time_attr_id: src.get_a_effect(&RAH_EFFECT_ID).and_then(|v| v.duration_attr_id),
            sim_running: false,
        }
    }
}
