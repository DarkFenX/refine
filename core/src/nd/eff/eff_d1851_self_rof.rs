use crate::{ac, ad, ec, ed, nd::NEffect};

const E_EFFECT_ID: ed::EEffectId = ec::effects::SELF_ROF;
const A_EFFECT_ID: ad::AEffectId = ac::effects::SELF_ROF;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: self-skillreq missile rof effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    let modifier = ad::AEffectModifier {
        affector_attr_id: ac::attrs::ROF_BONUS,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::LocSrq(ad::AEffectLocation::Ship, ad::AModifierSrq::SelfRef),
        affectee_attr_id: ac::attrs::SPEED,
    };
    a_effect.mods.push(modifier);
    a_effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
}
