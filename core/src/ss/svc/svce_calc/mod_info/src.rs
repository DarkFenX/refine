use crate::defs::{AttrVal, EAttrId, SsItemId};

#[derive(Debug)]
pub struct ModSrcInfo {
    pub item_id: SsItemId,
    pub val: ModSrcValInfo,
}
impl ModSrcInfo {
    pub(in crate::ss::svc::svce_calc) fn new(item_id: SsItemId, val: ModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}

#[derive(Debug)]
pub enum ModSrcValInfo {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
}
