use crate::{
    defs::{AttrVal, EItemCatId},
    shr::{ModAggrMode, ModOp},
};

pub(in crate::ss::svc::svce_calc) struct Modification {
    pub(in crate::ss::svc::svce_calc) op: ModOp,
    pub(in crate::ss::svc::svce_calc) val: AttrVal,
    pub(in crate::ss::svc::svce_calc) res_val: AttrVal,
    pub(in crate::ss::svc::svce_calc) aggr_mode: ModAggrMode,
    pub(in crate::ss::svc::svce_calc) src_item_cat_id: EItemCatId,
}
impl Modification {
    pub(in crate::ss::svc::svce_calc) fn new(
        op: ModOp,
        val: AttrVal,
        res_val: AttrVal,
        aggr_mode: ModAggrMode,
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
