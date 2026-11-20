use crate::{
    ad::{AEffect, AEffectCatId, AEffectId, AState},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::ONLINE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    // In EVE, "online" effect has active category, and it receives active state because of it. In
    // refine, highest module state is picked by its highest effect state. Modules which can never
    // be active (like plates) can become active without this fix
    match a_effect.category == AEffectCatId::ONLINE && a_effect.state == AState::Online {
        true => {
            let warning = format!("effect {EFFECT_AID}: online effect didn't need fixing");
            a_warnings.push(warning);
        }
        false => {
            a_effect.category = AEffectCatId::ONLINE;
            a_effect.state = AState::Online;
        }
    }
}
