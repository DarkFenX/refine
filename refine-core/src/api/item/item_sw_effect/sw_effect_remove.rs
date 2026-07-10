use crate::{
    api::SwEffectMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_sw_effect(
        &mut self,
        sw_effect_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, reuse_eupdates);
        self.u_data.sw_effects.remove(&sw_effect_uid);
        self.u_data.items.remove(sw_effect_uid);
    }
}

impl<'a> SwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_sw_effect(self.uid, &mut reuse_eupdates);
    }
}
