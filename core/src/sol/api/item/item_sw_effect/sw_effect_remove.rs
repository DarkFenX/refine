use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::SwEffectMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_sw_effect(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_sw_effect(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        self.uad.sw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> SwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_sw_effect(self.key, &mut reuse_eupdates);
    }
}
