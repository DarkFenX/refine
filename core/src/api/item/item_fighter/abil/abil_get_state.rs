use crate::{
    ad::AAbilId,
    api::{Ability, AbilityMut},
    misc::EffectMode,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Ability<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.fighter_key, &self.abil_id)
    }
}

impl<'a> AbilityMut<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.fighter_key, &self.abil_id)
    }
}

fn get_state(sol: &SolarSystem, fighter_key: UItemId, abil_id: &AAbilId) -> bool {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability(abil_id).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_key).dc_fighter().unwrap();
    match u_fighter.get_effect_key_mode(&r_abil.effect_key) {
        // Default active effects are ran if fighter becomes active, others are not
        EffectMode::FullCompliance => Some(r_abil.effect_key) == u_fighter.get_defeff_key().unwrap(),
        EffectMode::StateCompliance => true,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}
