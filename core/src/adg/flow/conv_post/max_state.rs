use crate::{ad, util::RMap};

pub(in crate::adg::flow::conv_post) fn fill_max_state(a_data: &mut ad::AData) {
    for a_item in a_data.items.values_mut() {
        a_item.max_state = get_max_state(a_item.effect_datas.keys(), &a_data.effects);
    }
}

fn get_max_state<'a>(
    item_effects: impl Iterator<Item = &'a ad::AEffectId>,
    effects: &RMap<ad::AEffectId, ad::AEffect>,
) -> ad::AState {
    let mut max_state = ad::AState::Offline;
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
