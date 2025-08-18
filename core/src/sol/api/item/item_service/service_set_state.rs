use crate::{
    misc::ServiceState,
    sol::{SolarSystem, api::ServiceMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_service_state(
        &mut self,
        service_key: UItemKey,
        state: ServiceState,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_service = self.u_data.items.get_mut(service_key).get_service_mut().unwrap();
        let old_a_state = u_service.get_state();
        u_service.set_service_state(state);
        let new_a_state = u_service.get_state();
        u_service.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            service_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ServiceMut<'a> {
    pub fn set_state(&mut self, state: ServiceState) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_service_state(self.key, state, &mut reuse_eupdates)
    }
}
