use crate::{
    rd::RAttrKey,
    svc::calc::{CalcAttrVal, ItemAttrPostprocs},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrValData {
    pub(in crate::svc::calc) values: RMap<RAttrKey, CalcAttrVal>,
    pub(in crate::svc::calc) postprocs: RMap<RAttrKey, ItemAttrPostprocs>,
}
impl ItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: RMap::new(),
            postprocs: RMap::new(),
        }
    }
}
