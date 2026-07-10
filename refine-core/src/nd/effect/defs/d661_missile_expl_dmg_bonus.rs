use super::shared::missile_dmg_self_srq_update_effect;
use crate::{
    ad::{AAttrId, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::MISSILE_EXPL_DMG_BONUS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect, a_warnings| {
            missile_dmg_self_srq_update_effect(EFFECT_AID, a_effect, a_warnings, AAttrId::EXPL_DMG)
        }),
        ..
    }
}
