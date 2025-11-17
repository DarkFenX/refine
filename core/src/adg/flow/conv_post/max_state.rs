use crate::{
    ad::{AData, AEffect, AEffectId, AState},
    util::RMap,
};

pub(in crate::adg::flow::conv_post) fn fill_max_state(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        a_item.max_state = get_max_state(a_item.effect_datas.keys(), &a_data.effects);
    }
}

fn get_max_state<'a>(item_effects: impl Iterator<Item = &'a AEffectId>, effects: &RMap<AEffectId, AEffect>) -> AState {
    let mut max_state = AState::Offline;
    for effect_id in item_effects {
        let effect = match effects.get(effect_id) {
            Some(effect) => effect,
            None => continue,
        };
        if effect.state > max_state {
            max_state = effect.state;
        }
    }
    max_state
}
