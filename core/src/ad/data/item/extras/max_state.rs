use crate::{
    ad::{AEffect, AEffectId, AState},
    util::StMap,
};

pub(super) fn get_max_state<'a>(
    item_effects: impl Iterator<Item = &'a AEffectId>,
    effects: &StMap<AEffectId, &AEffect>,
) -> AState {
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
