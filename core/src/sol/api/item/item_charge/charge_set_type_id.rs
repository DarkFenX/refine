use crate::{
    ad,
    sol::{ItemKey, ItemTypeId, SolarSystem, api::ChargeMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_charge_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::unload_charge(&mut self.svc, &self.uad, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_charge_mut()
            .unwrap()
            .set_a_item_id(&self.uad.src, a_item_id);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::load_charge(&mut self.svc, &self.uad, item_key, uad_item);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_charge_a_item_id(self.key, type_id)
    }
}
