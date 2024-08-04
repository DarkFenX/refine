use std::collections::HashMap;

pub(in crate::cmd) type HSideEffectMap = HashMap<rc::EEffectId, bool>;

pub(in crate::cmd) fn apply_side_effects(
    core_sol: &mut rc::SolarSystem,
    item_id: &rc::SolItemId,
    side_effects: &Option<HSideEffectMap>,
) -> rc::Result<()> {
    if let Some(side_effect_map) = side_effects {
        for (effect_id, status) in side_effect_map.iter() {
            core_sol.set_booster_side_effect_state(item_id, effect_id, *status)?;
        }
    }
    Ok(())
}
