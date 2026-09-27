use crate::{
    api::{ServiceMut, ServiceState},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_service_state(&mut self, service_uid: UItemId, state: ServiceState) {
        let u_service = self.u_data.items.get_mut(service_uid).dc_service_mut().unwrap();
        let old_a_state = u_service.get_state();
        u_service.set_service_state(state);
        let new_a_state = u_service.get_state();
        u_service.update_reffs(&mut self.cache.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            service_uid,
            old_a_state,
            new_a_state,
            &self.cache.eupdates,
        );
    }
}

impl<'s> ServiceMut<'s> {
    pub fn set_state(&mut self, state: ServiceState) {
        self.sol.internal_set_service_state(self.uid, state)
    }
}
