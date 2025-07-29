use crate::{ad::AAbilId, def::Count, sol::SolarSystem, ud::UItemKey};

/// Fighter ability.
pub struct Ability<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) item_key: UItemKey,
    pub(in crate::sol::api) abil_id: AAbilId,
}
impl<'a> Ability<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, item_key: UItemKey, abil_id: AAbilId) -> Self {
        Self { sol, item_key, abil_id }
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.item_key, &self.abil_id)
    }
}

/// Fighter ability which allows changing its state.
pub struct AbilityMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) item_key: UItemKey,
    pub(in crate::sol::api) abil_id: AAbilId,
}
impl<'a> AbilityMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, item_key: UItemKey, abil_id: AAbilId) -> Self {
        Self { sol, item_key, abil_id }
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.item_key, &self.abil_id)
    }
}

fn get_charge_count(sol: &SolarSystem, fighter_key: UItemKey, abil_id: &AAbilId) -> Option<Count> {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability(abil_id).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_key).get_fighter().unwrap();
    u_fighter
        .get_effect_datas()?
        .get(&r_abil.get_effect_key())?
        .charge_count
}
