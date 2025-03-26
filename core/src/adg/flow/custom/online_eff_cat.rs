use crate::{ac, ad};

const ONLINE_EFFECT: ad::AEffectId = ac::effects::ONLINE;

pub(in crate::adg::flow::custom) fn fix_online_effect_cat(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&ONLINE_EFFECT) {
        Some(effect) => match effect.state {
            ad::AState::Online => tracing::info!("\"online\" effect {ONLINE_EFFECT} category did not need fixing"),
            _ => effect.state = ad::AState::Online,
        },
        None => tracing::info!("\"online\" effect {ONLINE_EFFECT} is not found for fixing"),
    }
}
