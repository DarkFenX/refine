use crate::sol::{
    ItemId,
    svc::calc::{AttrValInfo, Calc, CalcAttrVal},
    uad::Uad,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fast: fn(&mut Calc, &Uad, &ItemId, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::sol::svc::calc) info: fn(&mut Calc, &Uad, &ItemId, AttrValInfo) -> AttrValInfo,
}
impl ItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fn new(
        fast: fn(&mut Calc, &Uad, &ItemId, CalcAttrVal) -> CalcAttrVal,
        info: fn(&mut Calc, &Uad, &ItemId, AttrValInfo) -> AttrValInfo,
    ) -> Self {
        Self { fast, info }
    }
}
