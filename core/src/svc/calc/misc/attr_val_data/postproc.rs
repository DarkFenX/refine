use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemId,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrPostprocs {
    pub(in crate::svc::calc) fast: fn(&mut Calc, SvcCtx, UItemId, CalcAttrVals) -> CalcAttrVals,
    pub(in crate::svc::calc) info: fn(&mut Calc, SvcCtx, UItemId, AttrValInfo) -> AttrValInfo,
}
