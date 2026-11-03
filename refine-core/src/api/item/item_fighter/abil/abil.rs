use crate::{ad::AAbilId, api::AbilityId, num::Count, sol::SolarSystem, ud::UItemId};

/// Fighter ability which allows changing its state.
pub struct Ability<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) fighter_uid: UItemId,
    pub(super) abil_aid: AAbilId,
}
impl<'s> Ability<'s> {
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
        let abil_aid = &self.abil_aid;
        // Only abilities which exist in source are exposed by API, just unwrap
        let r_abil = self.sol.u_data.r_data.get_ability_by_aid(abil_aid).unwrap();
        let u_fighter = self.sol.u_data.items.get(self.fighter_uid).dc_fighter().unwrap();
        u_fighter
            .get_r_item_base()?
            .effects
            .get(&r_abil.effect_rid)?
            .ability_charge_count
    }
}
