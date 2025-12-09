use crate::svc::calc::{CalcAttrVal, ItemAttrPostprocs};

#[derive(Clone)]
pub(in crate::svc::calc) struct AttrEntry {
    pub(in crate::svc::calc) value: Option<CalcAttrVal>,
    pub(in crate::svc::calc) postprocs: Option<ItemAttrPostprocs>,
}
impl AttrEntry {
    pub(in crate::svc::calc) fn is_empty(&self) -> bool {
        self.value.is_none() && self.postprocs.is_none()
    }
}
