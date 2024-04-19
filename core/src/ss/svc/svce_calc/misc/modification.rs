use crate::{
    defs::{AttrVal, EItemCatId},
    ss::svc::svce_calc::{SsModAggrMode, SsModOp},
};

pub(in crate::ss::svc::svce_calc) struct Modification {
    pub(in crate::ss::svc::svce_calc) op: SsModOp,
    pub(in crate::ss::svc::svce_calc) val: AttrVal,
    pub(in crate::ss::svc::svce_calc) res_val: AttrVal,
    pub(in crate::ss::svc::svce_calc) aggr_mode: SsModAggrMode,
    pub(in crate::ss::svc::svce_calc) src_item_cat_id: EItemCatId,
}
impl Modification {
    pub(in crate::ss::svc::svce_calc) fn new(
        op: SsModOp,
        val: AttrVal,
        res_val: AttrVal,
        aggr_mode: SsModAggrMode,
        src_item_cat_id: EItemCatId,
    ) -> Self {
        Self {
            op,
            val,
            res_val,
            aggr_mode,
            src_item_cat_id,
        }
    }
}
