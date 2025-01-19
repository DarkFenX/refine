use crate::{ad, defs::EEffectId, ec};

const MISSILE_ROF_EFFECT: EEffectId = ec::effects::SELF_ROF;

pub(in crate::adg::custom) fn mk_self_skillreq_modifiers_launcher_rof(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == MISSILE_ROF_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile rof effect {MISSILE_ROF_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectModifier::new(
            ec::attrs::ROF_BONUS,
            ad::AOp::PostPerc,
            ad::AEffectAffecteeFilter::LocSrq(ad::AEffectLocation::Ship, ad::AModifierSrq::SelfRef),
            ec::attrs::SPEED,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("self-skillreq missile rof effect {MISSILE_ROF_EFFECT} is not found for customization");
    }
}
