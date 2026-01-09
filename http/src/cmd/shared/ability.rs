use serde::Deserialize;
use serde_with::{Map, serde_as};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HAbilityMap {
    #[serde_as(as = "Map<_, _>")]
    states: Vec<(i32, bool)>,
}

pub(in crate::cmd) fn apply_abilities(core_fighter: &mut rc::FighterMut, abilities: &Option<HAbilityMap>) {
    if let Some(abilities) = abilities {
        // Apply state changes only to existing abilities, ignore the rest
        for (abil_id, new_state) in abilities.states.iter() {
            if let Ok(mut core_ability) = core_fighter.get_ability_mut(&rc::AbilId::from_i32(*abil_id)) {
                core_ability.set_state(*new_state);
            }
        }
    }
}
