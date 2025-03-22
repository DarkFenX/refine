use crate::{ac, ad};

const AB_EFFECT: ad::AEffectId = ac::effects::MOD_BONUS_AFTERBURNER;
const MWD_EFFECT: ad::AEffectId = ac::effects::MOD_BONUS_MICROWARPDRIVE;

pub(in crate::adg::flow::custom) fn add_ab_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == AB_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("slot modifier effect {AB_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect.mods.push(mk_mass_mod());
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {AB_EFFECT} is not found for customization");
    }
}

pub(in crate::adg::flow::custom) fn add_mwd_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == MWD_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("slot modifier effect {MWD_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect.mods.push(mk_mass_mod());
        effect.mods.push(mk_sig_mod());
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {MWD_EFFECT} is not found for customization");
    }
}

fn mk_mass_mod() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::MASS_ADDITION,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::MASS,
    }
}

fn mk_sig_mod() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    }
}
