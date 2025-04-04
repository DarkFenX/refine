use crate::{
    ad,
    sol::svc::calc::{CalcAttrVal, ItemAttrPostprocs},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrValData {
    pub(in crate::sol::svc::calc) values: RMap<ad::AAttrId, CalcAttrVal>,
    pub(in crate::sol::svc::calc) postprocs: RMap<ad::AAttrId, ItemAttrPostprocs>,
}
impl ItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: RMap::new(),
            postprocs: RMap::new(),
        }
    }
}
