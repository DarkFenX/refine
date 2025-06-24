use crate::sol::{
    ItemKey,
    svc::{
        calc::{AttrValInfo, Calc, CalcAttrVal},
        eprojs::EProjs,
    },
    uad::Uad,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fast: fn(&mut Calc, &Uad, &EProjs, ItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::sol::svc::calc) info: fn(&mut Calc, &Uad, &EProjs, ItemKey, AttrValInfo) -> AttrValInfo,
}
