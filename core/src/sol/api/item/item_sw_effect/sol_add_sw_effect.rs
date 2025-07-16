use crate::{
    ad,
    def::{ItemKey, ItemTypeId},
    sol::{SolarSystem, api::SwEffectMut},
    uad::{UadEffectUpdates, UadItem, UadSwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId) -> SwEffectMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.internal_add_sw_effect(type_id, &mut reuse_eupdates);
        SwEffectMut::new(self, item_key)
    }
    pub(in crate::sol::api) fn internal_add_sw_effect(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_sw_effect = UadSwEffect::new(item_id, a_item_id, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::SwEffect(uad_sw_effect);
        let item_key = self.uad.items.add(uad_item);
        self.uad.sw_effects.insert(item_key);
        SolarSystem::util_add_sw_effect(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}
