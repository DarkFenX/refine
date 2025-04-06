use crate::sol::{
    ItemTypeId, SolarSystem,
    info::ProjEffectInfo,
    uad::item::{Item, ProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId, state: bool) -> ProjEffectInfo {
        let item_id = self.uad.items.alloc_item_id();
        let proj_effect = ProjEffect::new(&self.uad.src, item_id, type_id, state);
        let info = ProjEffectInfo::from(&proj_effect);
        let item = Item::ProjEffect(proj_effect);
        self.uad.proj_effects.insert(item_id);
        self.uad.items.add(item);
        self.add_item_id_to_svc(&item_id);
        info
    }
}
