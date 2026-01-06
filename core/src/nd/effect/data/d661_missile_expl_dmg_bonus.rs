use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::missile_dmg_self_srq::update_effect},
};

const EFFECT_EID: EEffectId = EEffectId::MISSILE_EXPL_DMG_BONUS;
const EFFECT_AID: AEffectId = AEffectId::MISSILE_EXPL_DMG_BONUS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| update_effect(EFFECT_AID, a_effect, AAttrId::EXPL_DMG)),
        ..
    }
}
