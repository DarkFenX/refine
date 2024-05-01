use crate::defs::{AttrVal, EAttrId, SolItemId};

pub struct SolAffectorInfo {
    pub item_id: SolItemId,
    pub val: SolAffectorValueInfo,
}
impl SolAffectorInfo {
    pub(in crate::sol::svc::svce_calc) fn new(item_id: SolItemId, val: SolAffectorValueInfo) -> Self {
        Self { item_id, val }
    }
}

pub enum SolAffectorValueInfo {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
}
