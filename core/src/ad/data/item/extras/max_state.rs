use crate::{
    ad::{AEffect, AState},
    defs::EEffectId,
    util::StMap,
};

pub(super) fn get_max_state<'a>(
    item_effects: impl Iterator<Item = &'a EEffectId>,
    effects: &StMap<EEffectId, &AEffect>,
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
