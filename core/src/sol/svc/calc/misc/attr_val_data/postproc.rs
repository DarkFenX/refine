use crate::sol::{
    ItemKey,
    svc::calc::{AttrValInfo, Calc, CalcAttrVal},
    uad::Uad,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fast: fn(&mut Calc, &Uad, ItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::sol::svc::calc) info: fn(&mut Calc, &Uad, ItemKey, AttrValInfo) -> AttrValInfo,
}
