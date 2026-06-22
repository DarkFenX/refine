use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HAbilityMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    states: Vec<(i32, bool)>,
}
impl HAbilityMap {
    pub(in crate::cmd) fn apply(&self, core_fighter: &mut rc::FighterMut) {
        // Apply state changes only to existing abilities, ignore the rest
        for (abil_id, new_state) in self.states.iter() {
            if let Ok(mut core_ability) = core_fighter.get_ability_mut(&rc::AbilId::from_i32(*abil_id)) {
                core_ability.set_state(*new_state);
            }
        }
    }
}
