use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemKey,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::svc::calc) fast: fn(&mut Calc, SvcCtx, UItemKey, CalcAttrVals) -> CalcAttrVals,
    pub(in crate::svc::calc) info: fn(&mut Calc, SvcCtx, UItemKey, AttrValInfo) -> AttrValInfo,
}
