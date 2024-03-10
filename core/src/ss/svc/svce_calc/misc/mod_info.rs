use crate::{
    defs::{AttrVal, EAttrId, SsItemId},
    shr::{ModAggrMode, ModOp},
};

pub struct ModificationInfo {
    pub src_item_id: SsItemId,
    pub src_attr_id: Option<EAttrId>,
    pub val: AttrVal,
    pub op: ModOp,
    pub penalized: bool,
    pub aggr_mode: ModAggrMode,
}
impl ModificationInfo {
    pub(in crate::ss::svc::svce_calc) fn new(
        src_item_id: SsItemId,
        src_attr_id: Option<EAttrId>,
        val: AttrVal,
        op: ModOp,
        penalized: bool,
        aggr_mode: ModAggrMode,
    ) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            val,
            op,
            penalized,
            aggr_mode,
        }
    }
}
