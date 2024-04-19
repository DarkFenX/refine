use crate::defs::{AttrVal, EAttrId, SsItemId};

pub struct SsModSrcInfo {
    pub item_id: SsItemId,
    pub val: SsModSrcValInfo,
}
impl SsModSrcInfo {
    pub(in crate::ss::svc::svce_calc) fn new(item_id: SsItemId, val: SsModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}

pub enum SsModSrcValInfo {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
}
