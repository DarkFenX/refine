use crate::{ac, ad};

const DRONE_DMG_EFFECT: ad::AEffectId = ac::effects::DRONE_DMG_BONUS;

pub(in crate::adg::flow::custom) fn mk_self_skillreq_drone_dmg(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == DRONE_DMG_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq drone dmg effect {DRONE_DMG_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectModifier {
            affector_attr_id: ac::attrs::DMG_MULT_BONUS,
            op: ad::AOp::PostPerc,
            affectee_filter: ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::SelfRef),
            affectee_attr_id: ac::attrs::DMG_MULT,
        };
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("self-skillreq drone dmg effect {DRONE_DMG_EFFECT} is not found for customization");
    }
}
