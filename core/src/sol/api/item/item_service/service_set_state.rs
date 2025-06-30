use crate::{
    def::ItemKey,
    misc::ServiceState,
    sol::{SolarSystem, api::ServiceMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_service_state(&mut self, item_key: ItemKey, state: ServiceState) {
        let uad_service = self.uad.items.get_mut(item_key).get_service_mut().unwrap();
        let old_a_state = uad_service.get_a_state();
        uad_service.set_service_state(state);
        let new_a_state = uad_service.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
    }
}

impl<'a> ServiceMut<'a> {
    pub fn set_state(&mut self, state: ServiceState) {
        self.sol.internal_set_service_state(self.key, state)
    }
}
