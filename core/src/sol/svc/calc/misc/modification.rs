use crate::{
    ad,
    sol::{
        AttrVal,
        svc::calc::{AggrMode, Op},
    },
};

pub(in crate::sol::svc::calc) struct Modification {
    pub(in crate::sol::svc::calc) op: Op,
    pub(in crate::sol::svc::calc) val: AttrVal,
    pub(in crate::sol::svc::calc) res_mult: Option<AttrVal>,
    pub(in crate::sol::svc::calc) proj_mult: Option<AttrVal>,
    pub(in crate::sol::svc::calc) aggr_mode: AggrMode,
    pub(in crate::sol::svc::calc) affector_a_item_cat_id: ad::AItemCatId,
}
