use crate::{
    ad::{ADataGenerator, AEffect, AEffectId, AState},
    util::RMap,
};

impl ADataGenerator {
    pub(super) fn fill_max_state(&mut self) {
        for a_item in self.a_data.items.data.values_mut() {
            a_item.max_state = get_max_state(a_item.effects.keys(), &self.a_data.effects.data);
        }
    }
}

fn get_max_state<'a>(item_effects: impl Iterator<Item = &'a AEffectId>, effects: &RMap<AEffectId, AEffect>) -> AState {
    let mut max_state = AState::Offline;
    for effect_aid in item_effects {
        let Some(a_effect) = effects.get(effect_aid) else {
            continue;
        };
        if a_effect.state > max_state {
            max_state = a_effect.state;
        }
    }
    max_state
}
