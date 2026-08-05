use crate::{
    ad::AAttrId,
    nd::{NEffectChargeMultGetter, NEffectOutputGetter, NEffectProjMultGetter, NEffectResist},
};

pub(crate) struct NEffectProjOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter> = None,
    pub(crate) spoolable: bool = false,
    pub(crate) proj_mult_str: Option<NEffectProjMultGetter> = None,
    pub(crate) proj_mult_chance: Option<NEffectProjMultGetter> = None,
    pub(crate) resist: Option<NEffectResist> = None,
    pub(crate) local_limit_attr_id: Option<AAttrId> = None,
    pub(crate) remote_limit_attr_id: Option<AAttrId> = None,
}
