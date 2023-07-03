use crate::{ad, ec, shr::State};

pub(in crate::adg::custom) fn fix_online_effect_cat(a_data: &mut ad::AData) {
    let mut fixed = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == ec::effects::ONLINE) {
        if effect.state == State::Active {
            effect.state = State::Online;
            fixed = true;
        }
    }
    if !fixed {
        tracing::info!("\"online\" effect category did not need fixing")
    }
}
