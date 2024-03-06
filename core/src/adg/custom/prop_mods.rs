use crate::{
    ad,
    defs::EEffectId,
    ec,
    shr::{ModDomain, ModOp, State},
};

const AB_EFFECT: EEffectId = ec::effects::MOD_BONUS_AFTERBURNER;
const MWD_EFFECT: EEffectId = ec::effects::MOD_BONUS_MICROWARPDRIVE;

pub(in crate::adg::custom) fn add_ab_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == AB_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("slot modifier effect {AB_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect.mods.push(mk_mass_mod());
        effect.mod_build_status = ad::AModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {AB_EFFECT} isn't found for customization");
    }
}

pub(in crate::adg::custom) fn add_mwd_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == MWD_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("slot modifier effect {MWD_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect.mods.push(mk_mass_mod());
        effect.mods.push(mk_sig_mod());
        effect.mod_build_status = ad::AModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {MWD_EFFECT} isn't found for customization");
    }
}

fn mk_mass_mod() -> ad::AEffectAttrMod {
    ad::AEffectAttrMod::new(
        ec::attrs::MASS_ADDITION,
        ModOp::Add,
        ad::AModTgtFilter::Direct(ModDomain::Ship),
        ec::attrs::MASS,
    )
}

fn mk_sig_mod() -> ad::AEffectAttrMod {
    ad::AEffectAttrMod::new(
        ec::attrs::SIG_RADIUS_BONUS,
        ModOp::PostPerc,
        ad::AModTgtFilter::Direct(ModDomain::Ship),
        ec::attrs::SIG_RADIUS,
    )
}
