use crate::{
    def::ItemKey,
    misc::ServiceState,
    sol::{SolarSystem, api::ServiceMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_service_state(
        &mut self,
        item_key: ItemKey,
        state: ServiceState,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_service = self.uad.items.get_mut(item_key).get_service_mut().unwrap();
        let old_a_state = uad_service.get_a_state();
        uad_service.set_service_state(state, reuse_eupdates, &self.uad.src);
        let new_a_state = uad_service.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ServiceMut<'a> {
    pub fn set_state(&mut self, state: ServiceState) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_set_service_state(self.key, state, &mut reuse_eupdates)
    }
}
