use crate::{
    ad::AAttrId,
    svc::calc::{CalcAttrVal, ItemAttrPostprocs},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrValData {
    pub(in crate::svc::calc) values: RMap<AAttrId, CalcAttrVal>,
    pub(in crate::svc::calc) postprocs: RMap<AAttrId, ItemAttrPostprocs>,
}
impl ItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: RMap::new(),
            postprocs: RMap::new(),
        }
    }
}
