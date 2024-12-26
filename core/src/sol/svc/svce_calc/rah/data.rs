use crate::{
    defs::{EAttrId, SolFitId, SolItemId},
    sol::svc::svce_calc::SolAttrVal,
    util::{StMap, StMapSetL1},
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolRahSim {
    pub(super) resonances: StMap<SolItemId, StMap<EAttrId, SolAttrVal>>,
    pub(super) by_fit: StMapSetL1<SolFitId, SolItemId>,
    pub(super) running: bool,
}
impl SolRahSim {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
            running: false,
        }
    }
}
