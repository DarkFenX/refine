use crate::{
    defs::{AttrVal, EItemCatId},
    sol::svc::calc::{SolAggrMode, SolOp},
};

pub(in crate::sol::svc::calc) struct SolModification {
    pub(in crate::sol::svc::calc) op: SolOp,
    pub(in crate::sol::svc::calc) val: AttrVal,
    pub(in crate::sol::svc::calc) res_mult: Option<AttrVal>,
    pub(in crate::sol::svc::calc) proj_mult: Option<AttrVal>,
    pub(in crate::sol::svc::calc) aggr_mode: SolAggrMode,
    pub(in crate::sol::svc::calc) affector_item_cat_id: EItemCatId,
}
impl SolModification {
    pub(in crate::sol::svc::calc) fn new(
        op: SolOp,
        val: AttrVal,
        res_mult: Option<AttrVal>,
        proj_mult: Option<AttrVal>,
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
