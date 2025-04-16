use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    api::ProjEffectMut,
    uad::item::{UadItem, UadProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId, state: bool) -> ProjEffectMut {
        let item_key = self.internal_add_proj_effect(type_id, state);
        ProjEffectMut::new(self, item_key)
    }
    pub(in crate::sol) fn internal_add_proj_effect(&mut self, type_id: ItemTypeId, state: bool) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_proj_effect = UadProjEffect::new(&self.uad.src, item_id, type_id, state);
        let uad_item = UadItem::ProjEffect(uad_proj_effect);
        let item_key = self.uad.items.add(uad_item);
        self.uad.proj_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}
