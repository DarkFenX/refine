use crate::{ad::AEffectId, nd::NEffect};

const EFFECT_AID: AEffectId = AEffectId::CLONE_JUMP_ACCEPTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Effect has warp scram status modification, but is excluded from the tether check
        do_not_prevent_tether: true,
        ..
    }
}
