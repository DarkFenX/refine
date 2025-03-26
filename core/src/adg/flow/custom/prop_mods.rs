use crate::{ac, ad};

const AB_EFFECT: ad::AEffectId = ac::effects::MOD_BONUS_AFTERBURNER;
const MWD_EFFECT: ad::AEffectId = ac::effects::MOD_BONUS_MICROWARPDRIVE;

pub(in crate::adg::flow::custom) fn add_ab_modifiers(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&AB_EFFECT) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("slot modifier effect {AB_EFFECT} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(mk_mass_mod());
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("slot modifier effect {AB_EFFECT} is not found for customization"),
    }
}

pub(in crate::adg::flow::custom) fn add_mwd_modifiers(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&MWD_EFFECT) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("slot modifier effect {MWD_EFFECT} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(mk_mass_mod());
            effect.mods.push(mk_sig_mod());
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("slot modifier effect {MWD_EFFECT} is not found for customization"),
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
