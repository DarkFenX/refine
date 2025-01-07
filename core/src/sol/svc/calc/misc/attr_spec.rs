use crate::defs::{EAttrId, SolItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) struct SolAttrSpec {
    pub(in crate::sol::svc::calc) item_id: SolItemId,
    pub(in crate::sol::svc::calc) attr_id: EAttrId,
}
impl SolAttrSpec {
    pub(in crate::sol::svc::calc) fn new(item_id: SolItemId, attr_id: EAttrId) -> Self {
        Self { item_id, attr_id }
    }
}
