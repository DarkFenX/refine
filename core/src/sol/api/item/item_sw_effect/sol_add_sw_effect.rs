use crate::{
    ad,
    sol::{
        ItemKey, ItemTypeId, SolarSystem,
        api::SwEffectMut,
        uad::item::{UadItem, UadSwEffect},
    },
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId) -> SwEffectMut {
        let item_key = self.internal_add_sw_effect(type_id);
        SwEffectMut::new(self, item_key)
    }
    pub(in crate::sol::api) fn internal_add_sw_effect(&mut self, a_item_id: ad::AItemId) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_sw_effect = UadSwEffect::new(&self.uad.src, item_id, a_item_id, true);
        let uad_item = UadItem::SwEffect(uad_sw_effect);
        let item_key = self.uad.items.add(uad_item);
        self.uad.sw_effects.insert(item_key);
        self.internal_add_item_key_to_svc(item_key);
        item_key
    }
}
