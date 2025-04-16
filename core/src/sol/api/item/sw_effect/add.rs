use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    api::SwEffectMut,
    uad::item::{UadItem, UadSwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId, state: bool) -> SwEffectMut {
        let item_key = self.internal_add_sw_effect(type_id, state);
        SwEffectMut::new(self, item_key)
    }
    pub(in crate::sol) fn internal_add_sw_effect(&mut self, type_id: ItemTypeId, state: bool) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_sw_effect = UadSwEffect::new(&self.uad.src, item_id, type_id, state);
        let uad_item = UadItem::SwEffect(uad_sw_effect);
        let item_key = self.uad.items.add(uad_item);
        self.uad.sw_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}
