use crate::{
    defs::{EAttrId, SolFitId, SolItemId},
    sol::{svc::calc::SolAttrVal, SolDmgTypes},
    src::Src,
    util::{StMap, StMapSetL1},
};

use super::shared::RAH_EFFECT_ID;

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolRahSim {
    pub(super) resonances: StMap<SolItemId, Option<SolDmgTypes<SolAttrVal>>>,
    pub(super) by_fit: StMapSetL1<SolFitId, SolItemId>,
    pub(super) cycle_time_attr_id: Option<EAttrId>,
    pub(super) sim_running: bool,
}
impl SolRahSim {
    pub(in crate::sol::svc::calc) fn new(src: &Src) -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
            cycle_time_attr_id: src.get_a_effect(&RAH_EFFECT_ID).map(|v| v.duration_attr_id).flatten(),
            sim_running: false,
        }
    }
}
