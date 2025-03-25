use crate::sol::{
    ItemTypeId, SolarSystem,
    info::SwEffectInfo,
    uad::item::{Item, SwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId, state: bool) -> SwEffectInfo {
        let item_id = self.uad.items.alloc_item_id();
        let sw_effect = SwEffect::new(&self.uad.src, item_id, type_id, state);
        let info = SwEffectInfo::from(&sw_effect);
        let item = Item::SwEffect(sw_effect);
        self.uad.sw_effects.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        info
    }
}
