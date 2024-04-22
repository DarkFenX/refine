use crate::defs::{AttrVal, EAttrId, SolItemId};

pub struct SolModSrcInfo {
    pub item_id: SolItemId,
    pub val: SolModSrcValInfo,
}
impl SolModSrcInfo {
    pub(in crate::sol::svc::svce_calc) fn new(item_id: SolItemId, val: SolModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}

pub enum SolModSrcValInfo {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
}
