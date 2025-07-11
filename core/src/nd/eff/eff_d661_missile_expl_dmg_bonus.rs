use crate::{
    ac, ad, ec, ed,
    nd::{NEffect, eff::shared::missile_dmg_self_srq::update_effect},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::MISSILE_EXPL_DMG_BONUS;
const A_EFFECT_ID: ad::AEffectId = ac::effects::MISSILE_EXPL_DMG_BONUS;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect(A_EFFECT_ID, a_effect, ac::attrs::EXPL_DMG)),
        ..
    }
}
