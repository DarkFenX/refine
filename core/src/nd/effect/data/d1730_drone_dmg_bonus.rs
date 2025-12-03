use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::DRONE_DMG_BONUS;
const A_EFFECT_ID: AEffectId = ac::effects::DRONE_DMG_BONUS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: self-skillreq drone dmg effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: ac::attrs::DMG_MULT_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: ac::attrs::DMG_MULT,
    };
    a_effect.mods.push(modifier);
}
