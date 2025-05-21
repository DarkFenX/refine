use crate::sol::{ItemKey, SolarSystem, api::SubsystemMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_subsystem_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_subsystem = self.uad.items.get_mut(item_key).get_subsystem_mut().unwrap();
        let old_a_state = uad_subsystem.get_a_state();
        uad_subsystem.set_subsystem_state(state);
        let new_a_state = uad_subsystem.get_a_state();
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

impl<'a> SubsystemMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_subsystem_state(self.key, state)
    }
}
