use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjMultGetter, NModProjAttrsGetter},
};

const EFFECT_EID: EEffectId = EEffectId::from_i32(10_000_002);
const EFFECT_AID: AEffectId = AEffectId::from_eid(EFFECT_EID);

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        modifier_proj_attrs_getter: Some(NModProjAttrsGetter::Full),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::GenericRangeFullStsRestricted),
        ..
    }
}
