use crate::{ac, ad, ec, ed, ntt::NttEffect};

const E_EFFECT_ID: ed::EEffectId = ec::effects::SELF_ROF;
const A_EFFECT_ID: ad::AEffectId = ac::effects::SELF_ROF;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("self-skillreq missile rof effect {A_EFFECT_ID} has modifiers, overwriting them");
                effect.mods.clear();
            }
            let modifier = ad::AEffectModifier {
                affector_attr_id: ac::attrs::ROF_BONUS,
                op: ad::AOp::PostPerc,
                affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
                    ad::AEffectLocation::Ship,
                    ad::AModifierSrq::SelfRef,
                ),
                affectee_attr_id: ac::attrs::SPEED,
            };
            effect.mods.push(modifier);
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("self-skillreq missile rof effect {A_EFFECT_ID} is not found for customization"),
    }
}
