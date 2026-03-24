use crate::{
    ad::AAttrId,
    nd::{NEffectProjMultGetter, NEffectResist},
    num::{PValue, UnitInterval},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(crate) type NBaseOutputGetter<T, X> = fn(SvcCtx, &mut Calc, UItemId, &REffect, X) -> Option<Output<T>>;
pub(crate) type NChargeMultGetter = fn(SvcCtx, &mut Calc, UItemId, UnitInterval) -> Option<PValue>;

pub(crate) struct NEffectLocalOpcSpec<T, BX = ()>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T, BX>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}

pub(crate) struct NEffectProjOpcSpec<T, BX = ()>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T, BX>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) spoolable: bool = false,
    pub(crate) proj_mult_str: Option<NEffectProjMultGetter> = None,
    pub(crate) proj_mult_chance: Option<NEffectProjMultGetter> = None,
    pub(crate) resist: Option<NEffectResist> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}
