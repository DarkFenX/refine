use crate::{ad::AEffectId, ed::EEffectId, nd::NEffect};

const EFFECT_EID: EEffectId = EEffectId::CLOAKING_WARP_SAFE;
const EFFECT_AID: AEffectId = AEffectId::CLOAKING_WARP_SAFE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        cloaks_carrier: true,
        ..
    }
}
