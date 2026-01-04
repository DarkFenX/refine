use crate::{
    ad::AAbilId,
    def::{AbilId, DefCount},
    sol::SolarSystem,
    ud::UItemId,
};

/// Fighter ability.
pub struct Ability<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) fighter_key: UItemId,
    pub(in crate::api) abil_id: AAbilId,
}
impl<'a> Ability<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, fighter_key: UItemId, abil_id: AAbilId) -> Self {
        Self {
            sol,
            fighter_key,
            abil_id,
        }
    }
    pub fn get_id(&self) -> AbilId {
        self.abil_id
    }
    pub fn get_charge_count(&self) -> Option<DefCount> {
        get_charge_count(self.sol, self.fighter_key, &self.abil_id)
    }
}

/// Fighter ability which allows changing its state.
pub struct AbilityMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) fighter_key: UItemId,
    pub(in crate::api) abil_id: AAbilId,
}
impl<'a> AbilityMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, fighter_key: UItemId, abil_id: AAbilId) -> Self {
        Self {
            sol,
            fighter_key,
            abil_id,
        }
    }
    pub fn get_id(&self) -> AbilId {
        self.abil_id
    }
    pub fn get_charge_count(&self) -> Option<DefCount> {
        get_charge_count(self.sol, self.fighter_key, &self.abil_id)
    }
}

fn get_charge_count(sol: &SolarSystem, fighter_key: UItemId, abil_id: &AAbilId) -> Option<DefCount> {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability_by_aid(abil_id).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_key).dc_fighter().unwrap();
    u_fighter.get_effect_datas()?.get(&r_abil.effect_rid)?.charge_count
}
