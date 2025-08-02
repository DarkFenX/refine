use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::SwEffectMut},
    ud::{UEffectUpdates, UItem, UItemKey, USwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId) -> SwEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.internal_add_sw_effect(type_id, &mut reuse_eupdates);
        SwEffectMut::new(self, item_key)
    }
    pub(in crate::sol::api) fn internal_add_sw_effect(
        &mut self,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_sw_effect = USwEffect::new(item_id, type_id, true, &self.u_data.src);
        let u_item = UItem::SwEffect(u_sw_effect);
        let item_key = self.u_data.items.add(u_item);
        self.u_data.sw_effects.insert(item_key);
        SolarSystem::util_add_sw_effect(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}
