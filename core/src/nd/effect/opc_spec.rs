use crate::{
    ad::AAttrId,
    misc::{UnitInterval, Value},
    nd::{NEffectResist, NProjMultGetter},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(crate) type NBaseOutputGetter<T> = fn(SvcCtx, &mut Calc, UItemId, &REffect) -> Option<Output<T>>;
pub(crate) type NChargeMultGetter = fn(SvcCtx, &mut Calc, UItemId, UnitInterval) -> Option<Value>;

pub(crate) struct NEffectLocalOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}

pub(crate) struct NEffectProjOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) spoolable: bool = false,
    pub(crate) proj_mult_str: Option<NProjMultGetter> = None,
    pub(crate) proj_mult_chance: Option<NProjMultGetter> = None,
    pub(crate) resist: Option<NEffectResist> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}
