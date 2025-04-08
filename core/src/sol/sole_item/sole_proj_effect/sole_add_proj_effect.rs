use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    info::ProjEffectInfo,
    uad::item::{Item, ProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId, state: bool) -> ProjEffectInfo {
        let item_key = self.add_proj_effect_internal(type_id, state);
        self.get_proj_effect_internal(item_key).unwrap()
    }
    pub(in crate::sol) fn add_proj_effect_internal(&mut self, type_id: ItemTypeId, state: bool) -> ItemKey {
        let item_id = self.uad.items.alloc_item_id();
        let proj_effect = ProjEffect::new(&self.uad.src, item_id, type_id, state);
        let item = Item::ProjEffect(proj_effect);
        let item_key = self.uad.items.add(item);
        self.uad.proj_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}
