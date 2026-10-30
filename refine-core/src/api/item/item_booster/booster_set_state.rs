use crate::{
    api::BoosterMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_booster_state(&mut self, booster_uid: UItemId, state: bool) {
        let u_booster = self.u_data.items.get_mut(booster_uid).dc_booster_mut().unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::BoosterState(booster_uid, u_booster.get_booster_state()));
        }
        let old_a_state = u_booster.get_state();
        u_booster.set_booster_state(state);
        let new_a_state = u_booster.get_state();
        u_booster.update_reffs(&mut self.cache.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            booster_uid,
            old_a_state,
            new_a_state,
            &self.cache.eupdates,
        );
    }
}

impl<'s> BoosterMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_booster_state(self.uid, state)
    }
}
