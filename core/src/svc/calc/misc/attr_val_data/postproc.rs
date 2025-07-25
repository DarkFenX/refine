use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
    ud::UItemKey,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::svc::calc) fast: fn(&mut Calc, SvcCtx, UItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::svc::calc) info: fn(&mut Calc, SvcCtx, UItemKey, AttrValInfo) -> AttrValInfo,
}
