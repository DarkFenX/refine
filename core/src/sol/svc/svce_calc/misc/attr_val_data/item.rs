use crate::{
    defs::{EAttrId, SolItemId},
    sol::{
        svc::{svce_calc::SolAttrVal, SolSvc},
        uad::SolUad,
    },
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolItemAttrValData {
    pub(in crate::sol::svc::svce_calc) values: StMap<EAttrId, SolAttrVal>,
    pub(in crate::sol::svc::svce_calc) postprocessors:
        StMap<EAttrId, fn(&mut SolSvc, &SolUad, &SolItemId, SolAttrVal) -> SolAttrVal>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            postprocessors: StMap::new(),
        }
    }
}
