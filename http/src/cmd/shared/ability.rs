use std::collections::HashMap;

use rc::Lender;

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HAbilityMap {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    states: HashMap<rc::AbilId, bool>,
}

pub(in crate::cmd) fn apply_abilities(core_fighter: &mut rc::FighterMut, abilities: &Option<HAbilityMap>) {
    if let Some(ability_map) = abilities {
        // Apply state changes only to existing abilities, ignore the rest
        let mut ability_iter = core_fighter.iter_abilities_mut();
        while let Some(mut core_ability) = ability_iter.next() {
            if let Some(new_state) = ability_map.states.get(&core_ability.get_id()) {
                core_ability.set_state(*new_state);
            }
        }
    }
}
