use crate::{
    defs::{AttrVal, EItemCatId},
    sol::svc::svce_calc::{SolAggrMode, SolOp},
};

pub(in crate::sol::svc::svce_calc) struct SolModification {
    pub(in crate::sol::svc::svce_calc) op: SolOp,
    pub(in crate::sol::svc::svce_calc) val: AttrVal,
    pub(in crate::sol::svc::svce_calc) res_mult: AttrVal,
    pub(in crate::sol::svc::svce_calc) proj_mult: AttrVal,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolAggrMode,
    pub(in crate::sol::svc::svce_calc) affector_item_cat_id: EItemCatId,
}
impl SolModification {
    pub(in crate::sol::svc::svce_calc) fn new(
        op: SolOp,
        val: AttrVal,
        res_mult: AttrVal,
        proj_mult: AttrVal,
        aggr_mode: SolAggrMode,
        affector_item_cat_id: EItemCatId,
    ) -> Self {
        Self {
            op,
            val,
            res_mult,
            proj_mult,
            aggr_mode,
            affector_item_cat_id,
        }
    }
}
