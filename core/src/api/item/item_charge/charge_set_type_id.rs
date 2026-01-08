use crate::{
    ad::AItemId,
    api::{ChargeMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_charge_type_id(
        &mut self,
        charge_uid: UItemId,
        item_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(charge_uid);
        if u_item.get_type_id() == item_aid {
            return;
        }
        SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
        u_charge.set_type_id(item_aid, &self.u_data.src);
        SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
    }
}

impl<'a> ChargeMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_charge_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
