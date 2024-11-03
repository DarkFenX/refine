use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
};

const BUBBLE_EFFECT: EEffectId = ec::effects::WARP_DISRUPT_SPHERE;

pub(in crate::adg::custom) fn add_wdfg_modifiers(a_data: &mut ad::AData) {
    add_bubble_modifiers(a_data);
}

fn add_bubble_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == BUBBLE_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        // Signature radius
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::SIG_RADIUS_BONUS,
            ad::AOp::PostPerc,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Ship),
            ec::attrs::SIG_RADIUS,
        ));
        // Disallow assistance
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::DISALLOW_ASSISTANCE,
            ad::AOp::PostAssign,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Ship),
            ec::attrs::DISALLOW_ASSISTANCE,
        ));
        // Transfer scram strength to script
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::WARP_SCRAMBLE_STRENGTH,
            ad::AOp::PreAssign,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Other),
            ec::attrs::WARP_SCRAMBLE_STRENGTH,
        ));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} is not found for customization");
    }
}
