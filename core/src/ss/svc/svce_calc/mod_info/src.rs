use crate::defs::{AttrVal, EAttrId, SsItemId};

pub struct ModSrcInfo {
    pub item_id: SsItemId,
    pub val: ModSrcValInfo,
}
impl ModSrcInfo {
    pub(in crate::ss::svc::svce_calc) fn new(item_id: SsItemId, val: ModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}

pub enum ModSrcValInfo {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
}
