use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::ImplantMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_implant_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_implant = self.uad.items.get_mut(item_key).get_implant_mut().unwrap();
        let old_a_state = uad_implant.get_a_state();
        uad_implant.set_implant_state(state);
        let new_a_state = uad_implant.get_a_state();
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

impl<'a> ImplantMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_implant_state(self.key, state)
    }
}
