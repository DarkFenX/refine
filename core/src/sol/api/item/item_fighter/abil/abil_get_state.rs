use crate::{
    ad::AAbilId,
    misc::EffectMode,
    sol::{
        SolarSystem,
        api::{Ability, AbilityMut},
    },
    ud::UItemKey,
};

impl<'a> Ability<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.item_key, &self.abil_id)
    }
}

impl<'a> AbilityMut<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.item_key, &self.abil_id)
    }
}

fn get_state(sol: &SolarSystem, item_key: UItemKey, abil_id: &AAbilId) -> bool {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability(abil_id).unwrap();
    let u_fighter = sol.u_data.items.get(item_key).get_fighter().unwrap();
    match u_fighter.get_effect_key_mode(&r_abil.get_effect_key()) {
        // Default active effects are ran if fighter becomes active, others are not
        EffectMode::FullCompliance => Some(r_abil.get_effect_key()) == u_fighter.get_defeff_key().unwrap(),
        EffectMode::StateCompliance => true,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}
