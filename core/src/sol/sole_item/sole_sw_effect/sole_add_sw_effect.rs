use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    info::SwEffectInfo,
    uad::item::{UadItem, UadSwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId, state: bool) -> SwEffectInfo {
        let item_key = self.add_sw_effect_internal(type_id, state);
        self.get_sw_effect_info_internal(item_key).unwrap()
    }
    pub(in crate::sol) fn add_sw_effect_internal(&mut self, type_id: ItemTypeId, state: bool) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let sw_effect = UadSwEffect::new(&self.uad.src, item_id, type_id, state);
        let item = UadItem::SwEffect(sw_effect);
        let item_key = self.uad.items.add(item);
        self.uad.sw_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}
