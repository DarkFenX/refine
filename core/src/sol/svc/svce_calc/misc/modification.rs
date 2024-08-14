use crate::{
    defs::{AttrVal, EItemCatId},
    sol::svc::svce_calc::{SolAggrMode, SolOp},
};

pub(in crate::sol::svc::svce_calc) struct SolModification {
    pub(in crate::sol::svc::svce_calc) op: SolOp,
    pub(in crate::sol::svc::svce_calc) val: AttrVal,
    pub(in crate::sol::svc::svce_calc) res_mult: Option<AttrVal>,
    pub(in crate::sol::svc::svce_calc) proj_mult: Option<AttrVal>,
    pub(in crate::sol::svc::svce_calc) min_limit: Option<AttrVal>,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolAggrMode,
    pub(in crate::sol::svc::svce_calc) affector_item_cat_id: EItemCatId,
}
impl SolModification {
    pub(in crate::sol::svc::svce_calc) fn new(
        op: SolOp,
        val: AttrVal,
        res_mult: Option<AttrVal>,
        proj_mult: Option<AttrVal>,
        min_limit: Option<AttrVal>,
        aggr_mode: SolAggrMode,
        affector_item_cat_id: EItemCatId,
    ) -> Self {
        Self {
            op,
            val,
            res_mult,
            proj_mult,
            min_limit,
            aggr_mode,
            affector_item_cat_id,
        }
    }
}
