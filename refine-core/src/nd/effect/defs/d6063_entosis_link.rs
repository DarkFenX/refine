use crate::{ad::AEffectId, nd::NEffect};

const EFFECT_AID: AEffectId = AEffectId::ENTOSIS_LINK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect { aid: EFFECT_AID, .. }
}
