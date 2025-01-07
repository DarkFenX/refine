use crate::defs::{EAttrId, SolItemId};

pub struct SolAffectorInfo {
    pub item_id: SolItemId,
    pub attr_id: Option<EAttrId>,
}
impl SolAffectorInfo {
    pub(in crate::sol::svc::calc) fn new(item_id: SolItemId, attr_id: Option<EAttrId>) -> Self {
        Self { item_id, attr_id }
    }
}
