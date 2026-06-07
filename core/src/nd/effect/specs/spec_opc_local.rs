use crate::{
    ad::AAttrId,
    nd::{NEffectChargeMultGetter, NEffectOutputGetter},
};

pub(crate) struct NEffectLocalOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter> = None,
    pub(crate) limit_attr_id: Option<AAttrId> = None,
}
