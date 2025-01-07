use crate::{
    defs::EItemId,
    sol::{
        info::SolSwEffectInfo,
        uad::item::{SolItem, SolSwEffect},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: EItemId, state: bool) -> SolSwEffectInfo {
        let item_id = self.uad.items.alloc_item_id();
        let sw_effect = SolSwEffect::new(&self.uad.src, item_id, type_id, state);
        let info = SolSwEffectInfo::from(&sw_effect);
        let item = SolItem::SwEffect(sw_effect);
        self.uad.sw_effects.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        info
    }
}
