use crate::defs::{EAttrId, SolItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolAttrSpec {
    pub(in crate::sol::svc::svce_calc) item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) attr_id: EAttrId,
}
impl SolAttrSpec {
    pub(super) fn new(item_id: SolItemId, attr_id: EAttrId) -> Self {
        Self { item_id, attr_id }
    }
}
