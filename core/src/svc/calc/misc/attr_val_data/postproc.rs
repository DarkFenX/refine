use crate::{
    def::ItemKey,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::svc::calc) fast: fn(&mut Calc, SvcCtx, ItemKey, CalcAttrVal) -> CalcAttrVal,
    pub(in crate::svc::calc) info: fn(&mut Calc, SvcCtx, ItemKey, AttrValInfo) -> AttrValInfo,
}
