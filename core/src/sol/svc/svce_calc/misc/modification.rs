use crate::{
    defs::{AttrVal, EItemCatId},
    sol::svc::svce_calc::{SolModAggrMode, SolModOp},
};

pub(in crate::sol::svc::svce_calc) struct SolModification {
    pub(in crate::sol::svc::svce_calc) op: SolModOp,
    pub(in crate::sol::svc::svce_calc) val: AttrVal,
    pub(in crate::sol::svc::svce_calc) res_val: AttrVal,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolModAggrMode,
    pub(in crate::sol::svc::svce_calc) src_item_cat_id: EItemCatId,
}
impl SolModification {
    pub(in crate::sol::svc::svce_calc) fn new(
        op: SolModOp,
        val: AttrVal,
        res_val: AttrVal,
        aggr_mode: SolModAggrMode,
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
