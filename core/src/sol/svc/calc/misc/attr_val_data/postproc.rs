use crate::sol::{
    ItemKey,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::sol::svc::calc) fast: fn(&mut Calc, &SvcCtx, ItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::sol::svc::calc) info: fn(&mut Calc, &SvcCtx, ItemKey, AttrValInfo) -> AttrValInfo,
}
