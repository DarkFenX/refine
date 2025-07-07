use crate::{
    ad,
    def::AttrVal,
    svc::calc::{AggrMode, Op},
};

pub(in crate::svc::calc) struct Modification {
    pub(in crate::svc::calc) op: Op,
    pub(in crate::svc::calc) val: AttrVal,
    pub(in crate::svc::calc) proj_mult: Option<AttrVal>,
    pub(in crate::svc::calc) res_mult: Option<AttrVal>,
    pub(in crate::svc::calc) aggr_mode: AggrMode,
    pub(in crate::svc::calc) affector_a_item_cat_id: ad::AItemCatId,
}
