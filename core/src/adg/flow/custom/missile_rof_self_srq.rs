use crate::{ac, ad};

const MISSILE_ROF_EFFECT: ad::AEffectId = ac::effects::SELF_ROF;

pub(in crate::adg::flow::custom) fn mk_self_skillreq_modifiers_launcher_rof(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&MISSILE_ROF_EFFECT) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("self-skillreq missile rof effect {MISSILE_ROF_EFFECT} has modifiers, overwriting them");
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
        None => tracing::info!("self-skillreq missile rof effect {MISSILE_ROF_EFFECT} is not found for customization"),
    }
}
