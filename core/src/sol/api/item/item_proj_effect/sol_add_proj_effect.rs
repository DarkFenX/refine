use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::ProjEffectMut},
    ud::{UEffectUpdates, UItem, UItemKey, UProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId) -> ProjEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let proj_effect_key = self.internal_add_proj_effect(type_id, &mut reuse_eupdates);
        ProjEffectMut::new(self, proj_effect_key)
    }
    pub(in crate::sol::api) fn internal_add_proj_effect(
        &mut self,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_proj_effect = UProjEffect::new(item_id, type_id, true, &self.u_data.src);
        let u_item = UItem::ProjEffect(u_proj_effect);
        let proj_effect_key = self.u_data.items.add(u_item);
        self.u_data.proj_effects.insert(proj_effect_key);
        SolarSystem::util_add_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_key, reuse_eupdates);
        proj_effect_key
    }
}
