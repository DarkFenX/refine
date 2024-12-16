use crate::{
    defs::EItemId,
    sol::{
        item::{SolItem, SolProjEffect},
        item_info::SolProjEffectInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: EItemId, state: bool) -> SolProjEffectInfo {
        let item_id = self.items.alloc_item_id();
        let proj_effect = SolProjEffect::new(&self.src, item_id, type_id, state);
        let info = SolProjEffectInfo::from(&proj_effect);
        let item = SolItem::ProjEffect(proj_effect);
        self.proj_effects.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        info
    }
}
