use crate::{
    defs::{EAttrId, SolItemId},
    sol::{
        svc::calc::{SolAttrVal, SolCalc},
        uad::SolUad,
    },
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolItemAttrValData {
    pub(in crate::sol::svc::calc) values: StMap<EAttrId, SolAttrVal>,
    pub(in crate::sol::svc::calc) postprocessors:
        StMap<EAttrId, fn(&mut SolCalc, &SolUad, &SolItemId, SolAttrVal) -> SolAttrVal>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            postprocessors: StMap::new(),
        }
    }
}
