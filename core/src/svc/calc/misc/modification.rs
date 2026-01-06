use crate::{
    ad::AItemCatId,
    misc::{PValue, Value},
    svc::calc::{AggrMode, CalcOp},
};

pub(in crate::svc::calc) struct CalcModification {
    pub(in crate::svc::calc) op: CalcOp,
    pub(in crate::svc::calc) val: Value,
    pub(in crate::svc::calc) proj_mult: Option<PValue>,
    pub(in crate::svc::calc) res_mult: Option<PValue>,
    pub(in crate::svc::calc) aggr_mode: AggrMode,
    pub(in crate::svc::calc) affector_item_cat_id: AItemCatId,
}
