use crate::{
    ad,
    def::ItemTypeId,
    sol::{SolarSystem, api::ProjEffectMut},
    ud::{UEffectUpdates, UItem, UItemKey, UProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId) -> ProjEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.internal_add_proj_effect(type_id, &mut reuse_eupdates);
        ProjEffectMut::new(self, item_key)
    }
    pub(in crate::sol::api) fn internal_add_proj_effect(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_proj_effect = UProjEffect::new(item_id, a_item_id, true, &self.u_data.src, reuse_eupdates);
        let u_item = UItem::ProjEffect(u_proj_effect);
        let item_key = self.u_data.items.add(u_item);
        self.u_data.proj_effects.insert(item_key);
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        item_key
    }
}
