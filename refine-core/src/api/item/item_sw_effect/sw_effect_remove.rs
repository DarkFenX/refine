use crate::{api::SwEffectMut, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api::item) fn internal_remove_sw_effect(&mut self, sw_effect_uid: UItemId) {
        SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, &mut self.cache.eupdates);
        self.u_data.sw_effects.remove(&sw_effect_uid);
        self.u_data.items.remove(sw_effect_uid);
    }
}

impl<'s> SwEffectMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_sw_effect(self.uid);
    }
}
