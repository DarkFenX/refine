use crate::{ad::AEffectId, nd::NEffect};

const EFFECT_AID: AEffectId = AEffectId::CLOAKING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        cloaks_carrier: true,
        ..
    }
}
