use crate::{ad::AAbilId, api::AbilityId, num::Count, sol::SolarSystem, ud::UItemId};

/// Fighter ability.
pub struct Ability<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) fighter_uid: UItemId,
    pub(in crate::api) abil_aid: AAbilId,
}
impl<'s> Ability<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, fighter_uid: UItemId, abil_aid: AAbilId) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aid,
        }
    }
    pub fn get_id(&self) -> AbilityId {
        AbilityId::from_aid(self.abil_aid)
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

/// Fighter ability which allows changing its state.
pub struct AbilityMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) fighter_uid: UItemId,
    pub(in crate::api) abil_aid: AAbilId,
}
impl<'s> AbilityMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, fighter_uid: UItemId, abil_aid: AAbilId) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aid,
        }
    }
    pub fn get_id(&self) -> AbilityId {
        AbilityId::from_aid(self.abil_aid)
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

fn get_charge_count(sol: &SolarSystem, fighter_uid: UItemId, abil_aid: &AAbilId) -> Option<Count> {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.r_data.get_ability_by_aid(abil_aid).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    u_fighter.get_effects()?.get(&r_abil.effect_rid)?.charge_count
}
