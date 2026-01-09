use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HSideEffectMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(rc::EffectId, bool)>,
}

pub(in crate::cmd) fn apply_side_effects(core_booster: &mut rc::BoosterMut, side_effects: &Option<HSideEffectMap>) {
    if let Some(side_effect_map) = side_effects {
        for (effect_id, status) in side_effect_map.data.iter() {
            core_booster.get_side_effect_mut(effect_id).set_state(*status);
        }
    }
}
