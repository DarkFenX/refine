use crate::{ad::AAbilId, api::AbilId, num::Count, sol::SolarSystem, ud::UItemId};

/// Fighter ability.
pub struct Ability<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) fighter_uid: UItemId,
    pub(in crate::api) abil_aid: AAbilId,
}
impl<'a> Ability<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, fighter_uid: UItemId, abil_aid: AAbilId) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aid,
        }
    }
    pub fn get_id(&self) -> AbilId {
        AbilId::from_aid(self.abil_aid)
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

/// Fighter ability which allows changing its state.
pub struct AbilityMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) fighter_uid: UItemId,
    pub(in crate::api) abil_aid: AAbilId,
}
impl<'a> AbilityMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, fighter_uid: UItemId, abil_aid: AAbilId) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aid,
        }
    }
    pub fn get_id(&self) -> AbilId {
        AbilId::from_aid(self.abil_aid)
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.fighter_uid, &self.abil_aid)
    }
}

fn get_charge_count(sol: &SolarSystem, fighter_uid: UItemId, abil_aid: &AAbilId) -> Option<Count> {
    // Only abilities which exist in source are exposed by API, just unwrap
    let r_abil = sol.u_data.src.get_ability_by_aid(abil_aid).unwrap();
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    u_fighter.get_effect_datas()?.get(&r_abil.effect_rid)?.charge_count
}
