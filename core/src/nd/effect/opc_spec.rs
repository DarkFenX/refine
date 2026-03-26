use crate::{
    ad::AAttrId,
    nd::{NChargeMultGetter, NEffectProjMultGetter, NEffectResist, NOutputGetter},
};

pub(crate) struct NEffectLocalOpcSpec<BG>
where
    BG: NOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}

pub(crate) struct NEffectProjOpcSpec<BG>
where
    BG: NOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) spoolable: bool = false,
    pub(crate) proj_mult_str: Option<NEffectProjMultGetter> = None,
    pub(crate) proj_mult_chance: Option<NEffectProjMultGetter> = None,
    pub(crate) resist: Option<NEffectResist> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}
