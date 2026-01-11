use crate::{
    ad::{AData, AEffect, AEffectId, AState},
    util::RMap,
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_max_state(a_data: &mut AData) {
    for a_item in a_data.items.data.values_mut() {
        a_item.max_state = get_max_state(a_item.effect_datas.keys(), &a_data.effects.data);
    }
}

fn get_max_state<'a>(item_effects: impl Iterator<Item = &'a AEffectId>, effects: &RMap<AEffectId, AEffect>) -> AState {
    let mut max_state = AState::Offline;
    for effect_aid in item_effects {
        let a_effect = match effects.get(effect_aid) {
            Some(effect) => effect,
            None => continue,
        };
        if a_effect.state > max_state {
            max_state = a_effect.state;
        }
    }
    max_state
}
