use crate::{
    ad::AItemId,
    api::{ItemTypeId, StanceMut},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_stance_type_aid(&mut self, stance_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(stance_uid);
        if u_item.get_type_aid() == type_aid {
            return;
        }
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_uid, &mut self.cache.eupdates);
        let u_stance = self.u_data.items.get_mut(stance_uid).dc_stance_mut().unwrap();
        u_stance.set_type_aid(type_aid, &self.u_data.r_data);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_uid, &mut self.cache.eupdates);
    }
}

impl<'s> StanceMut<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_stance_type_aid(self.uid, type_id.into_aid())
    }
}
