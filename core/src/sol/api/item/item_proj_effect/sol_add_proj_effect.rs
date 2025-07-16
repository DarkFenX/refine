use crate::{
    ad,
    def::{ItemKey, ItemTypeId},
    sol::{SolarSystem, api::ProjEffectMut},
    uad::{UadEffectUpdates, UadItem, UadProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId) -> ProjEffectMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.internal_add_proj_effect(type_id, &mut reuse_eupdates);
        ProjEffectMut::new(self, item_key)
    }
    pub(in crate::sol::api) fn internal_add_proj_effect(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_proj_effect = UadProjEffect::new(item_id, a_item_id, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::ProjEffect(uad_proj_effect);
        let item_key = self.uad.items.add(uad_item);
        self.uad.proj_effects.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}
