use crate::{ad, consts};

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    fix_online_effect_cat(a_data);
}

fn fix_online_effect_cat(a_data: &mut ad::AData) {
    let mut fixed = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == consts::effects::ONLINE) {
        if effect.state == consts::State::Active {
            effect.state = consts::State::Online;
            fixed = true;
        }
    }
    if !fixed {
        tracing::info!("\"online\" effect category did not need fixing")
    }
}
