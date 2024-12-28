use crate::{
    defs::{SolFitId, SolItemId},
    sol::{svc::svce_calc::SolAttrVal, SolDmgTypes},
    util::{StMap, StMapSetL1},
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolRahSim {
    pub(super) resonances: StMap<SolItemId, Option<SolDmgTypes<SolAttrVal>>>,
    pub(super) by_fit: StMapSetL1<SolFitId, SolItemId>,
    pub(super) sim_running: bool,
}
impl SolRahSim {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
            sim_running: false,
        }
    }
}
