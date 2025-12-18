use crate::{
    ad::AItemCatId,
    def::AttrVal,
    svc::calc::{AggrMode, CalcOp},
};

pub(in crate::svc::calc) struct CalcModification {
    pub(in crate::svc::calc) op: CalcOp,
    pub(in crate::svc::calc) val: AttrVal,
    pub(in crate::svc::calc) proj_mult: Option<AttrVal>,
    pub(in crate::svc::calc) res_mult: Option<AttrVal>,
    pub(in crate::svc::calc) aggr_mode: AggrMode,
    pub(in crate::svc::calc) affector_item_cat_id: AItemCatId,
}
