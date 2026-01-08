use crate::{
    ad::AAbilId,
    api::{Ability, AbilityMut},
    misc::EffectMode,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Ability<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

impl<'a> AbilityMut<'a> {
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

fn get_state(sol: &SolarSystem, fighter_uid: UItemId, abil_id: &AAbilId) -> bool {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability_by_aid(abil_id).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    match u_fighter.get_effect_mode(&r_abil.effect_rid) {
        // Default active effects are ran if fighter becomes active, others are not
        EffectMode::FullCompliance => Some(r_abil.effect_rid) == u_fighter.get_defeff_rid().unwrap(),
        EffectMode::StateCompliance => true,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}
