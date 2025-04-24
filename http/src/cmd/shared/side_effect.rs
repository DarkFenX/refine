use std::collections::HashMap;

use crate::shared::HEffectId;

pub(in crate::cmd) type HSideEffectMap = HashMap<HEffectId, bool>;

pub(in crate::cmd) fn apply_side_effects(core_booster: &mut rc::BoosterMut, side_effects: &Option<HSideEffectMap>) {
    if let Some(side_effect_map) = side_effects {
        for (effect_id, status) in side_effect_map.iter() {
            core_booster.get_side_effect_mut(&effect_id.into()).set_state(*status);
        }
    }
}
