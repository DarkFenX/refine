use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, eff::shared::missile_dmg_self_srq::update_effect},
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_EM_DMG_BONUS;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_EM_DMG_BONUS;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect(A_EFFECT_ID, a_effect, ac::attrs::EM_DMG)),
        ..
    }
}
