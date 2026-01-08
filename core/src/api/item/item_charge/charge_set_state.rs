use crate::{
    api::ChargeMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_charge_state(
        &mut self,
        charge_uid: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
        let old_a_state = u_charge.get_state();
        u_charge.set_force_disabled(!state);
        let new_a_state = u_charge.get_state();
        u_charge.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            charge_uid,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ChargeMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_charge_state(self.uid, state, &mut reuse_eupdates)
    }
}
