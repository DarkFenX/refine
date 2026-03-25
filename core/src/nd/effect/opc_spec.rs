use crate::{
    ad::AAttrId,
    nd::{NEffectProjMultGetter, NEffectResist, NOutputGetter},
    num::{PValue, UnitInterval},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(crate) type NChargeMultGetter = fn(SvcCtx, &mut Calc, UItemId, UnitInterval) -> Option<PValue>;

pub(crate) struct NEffectLocalOpcSpec<GB>
where
    GB: NOutputGetter,
{
    pub(crate) base: GB,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}

pub(crate) struct NEffectProjOpcSpec<GB>
where
    GB: NOutputGetter,
{
    pub(crate) base: GB,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) spoolable: bool = false,
    pub(crate) proj_mult_str: Option<NEffectProjMultGetter> = None,
    pub(crate) proj_mult_chance: Option<NEffectProjMultGetter> = None,
    pub(crate) resist: Option<NEffectResist> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}
