use crate::{
    defs::SolItemId,
    sol::{
        svc::calc::{SolAttrVal, SolAttrValInfo, SolCalc},
        uad::SolUad,
    },
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fast: fn(&mut SolCalc, &SolUad, &SolItemId, SolAttrVal) -> SolAttrVal,
    pub(in crate::sol::svc::calc) info: fn(&mut SolCalc, &SolUad, &SolItemId, SolAttrValInfo) -> SolAttrValInfo,
}
impl SolItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fn new(
        fast: fn(&mut SolCalc, &SolUad, &SolItemId, SolAttrVal) -> SolAttrVal,
        info: fn(&mut SolCalc, &SolUad, &SolItemId, SolAttrValInfo) -> SolAttrValInfo,
    ) -> Self {
        Self { fast, info }
    }
}
