use crate::{
    defs::EAttrId,
    sol::svc::calc::{SolAttrVal, SolItemAttrPostprocs},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolItemAttrValData {
    pub(in crate::sol::svc::calc) values: StMap<EAttrId, SolAttrVal>,
    pub(in crate::sol::svc::calc) postprocs: StMap<EAttrId, SolItemAttrPostprocs>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            postprocs: StMap::new(),
        }
    }
}
