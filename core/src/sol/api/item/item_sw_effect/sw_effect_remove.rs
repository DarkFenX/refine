use crate::sol::{ItemKey, SolarSystem, api::SwEffectMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_sw_effect(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_item(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        self.uad.sw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> SwEffectMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_sw_effect(self.key);
    }
}
