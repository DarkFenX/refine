use crate::{
    ad,
    sol::svc::calc::{CalcAttrVal, ItemAttrPostprocs},
    util::HMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrValData {
    pub(in crate::sol::svc::calc) values: HMap<ad::AAttrId, CalcAttrVal>,
    pub(in crate::sol::svc::calc) postprocs: HMap<ad::AAttrId, ItemAttrPostprocs>,
}
impl ItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: HMap::new(),
            postprocs: HMap::new(),
        }
    }
}
