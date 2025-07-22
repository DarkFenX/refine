use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
    uad::UadItemKey,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::svc::calc) fast: fn(&mut Calc, SvcCtx, UadItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::svc::calc) info: fn(&mut Calc, SvcCtx, UadItemKey, AttrValInfo) -> AttrValInfo,
}
