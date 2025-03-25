use crate::{
    ad,
    sol::svc::calc::{CalcAttrVal, ItemAttrPostprocs},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrValData {
    pub(in crate::sol::svc::calc) values: StMap<ad::AAttrId, CalcAttrVal>,
    pub(in crate::sol::svc::calc) postprocs: StMap<ad::AAttrId, ItemAttrPostprocs>,
}
impl ItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            postprocs: StMap::new(),
        }
    }
}
