use crate::{
    defs::{AttrVal, EAttrId, SsItemId},
    shr::{ModAggrMode, ModOp},
};

pub struct ModificationInfo {
    pub src_item_id: SsItemId,
    pub src_attr_id: Option<EAttrId>,
    pub op: ModOp,
    pub aggr_mode: ModAggrMode,
    pub val: AttrVal,
}
impl ModificationInfo {
    pub(crate) fn new(
        src_item_id: SsItemId,
        src_attr_id: Option<EAttrId>,
        op: ModOp,
        aggr_mode: ModAggrMode,
        val: AttrVal,
    ) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            op,
            aggr_mode,
            val,
        }
    }
}
