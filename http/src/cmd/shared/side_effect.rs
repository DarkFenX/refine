use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HSideEffectMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(rc::EffectId, bool)>,
}
impl HSideEffectMap {
    pub(in crate::cmd) fn apply(&self, core_booster: &mut rc::BoosterMut) {
        for (effect_id, status) in self.data.iter() {
            if let Ok(mut core_side_effect) = core_booster.get_side_effect_mut(effect_id) {
                core_side_effect.set_state(*status);
            }
        }
    }
}
