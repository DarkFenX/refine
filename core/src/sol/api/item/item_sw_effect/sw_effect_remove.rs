use crate::{
    sol::{SolarSystem, api::SwEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_sw_effect(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        self.u_data.sw_effects.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> SwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_sw_effect(self.key, &mut reuse_eupdates);
    }
}
