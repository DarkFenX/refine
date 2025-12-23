use crate::svc::calc::{CalcAttrVals, ItemAttrPostprocs};

#[derive(Clone)]
pub(in crate::svc::calc) struct AttrEntry {
    pub(in crate::svc::calc) value: Option<CalcAttrVals>,
    pub(in crate::svc::calc) postprocs: Option<ItemAttrPostprocs>,
}
