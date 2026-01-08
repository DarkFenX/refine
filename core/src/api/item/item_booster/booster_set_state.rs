use crate::{
    api::BoosterMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_booster_state(
        &mut self,
        booster_uid: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_booster = self.u_data.items.get_mut(booster_uid).dc_booster_mut().unwrap();
        let old_a_state = u_booster.get_state();
        u_booster.set_booster_state(state);
        let new_a_state = u_booster.get_state();
        u_booster.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            booster_uid,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> BoosterMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_booster_state(self.uid, state, &mut reuse_eupdates)
    }
}
