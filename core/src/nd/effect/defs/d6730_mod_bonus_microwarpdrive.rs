use super::shared::{mk_mwd_sig_mod, mk_prop_mass_mod};
use crate::{
    ad::{AEffect, AEffectId},
    nd::NEffect,
    svc::calc::CalcCustomModifier,
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_MICROWARPDRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        calc_custom_mod: Some(CalcCustomModifier::PropSpeed),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, adg_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: MWD effect has modifiers, overwriting them");
        adg_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([mk_prop_mass_mod(), mk_mwd_sig_mod()]);
}
