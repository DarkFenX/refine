use crate::{ad, defs::EEffectId, ec};

const ONLINE_EFFECT: EEffectId = ec::effects::ONLINE;

pub(in crate::adg::custom) fn fix_online_effect_cat(a_data: &mut ad::AData) {
    let mut fixed = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == ONLINE_EFFECT) {
        if matches!(effect.state, ad::AState::Active) {
            effect.state = ad::AState::Online;
            fixed = true;
        }
    }
    if !fixed {
        tracing::info!("\"online\" effect {ONLINE_EFFECT} category did not need fixing")
    }
}
