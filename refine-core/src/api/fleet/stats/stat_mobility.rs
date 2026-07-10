use crate::{
    api::{AffectionDir, CtlAffectors, FleetMut},
    num::PValue,
    ud::UEffectUpdates,
    util::RMap,
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_mass(&mut self, ctl_affectors: CtlAffectors) -> PValue {
        let mut fleet_mass = PValue::ZERO;
        let mut saved_states = RMap::new();
        let mut reuse_eupdates = UEffectUpdates::new();
        let ship_uids = self.sol.u_data.get_fleet_ship_uids(self.uid);
        // Work on item states according to request
        for &ship_uid in ship_uids.iter() {
            self.sol.internal_ctl_affectors_switch(
                ship_uid,
                self.sol.u_data.r_data.get_attr_consts().mass,
                ctl_affectors,
                AffectionDir::Increase,
                &mut saved_states,
                &mut reuse_eupdates,
            );
        }
        // Collect stats
        for ship_uid in ship_uids.into_iter() {
            if let Ok(ship_mass) = self.sol.svc.get_stat_item_mass(&self.sol.u_data, ship_uid) {
                fleet_mass += ship_mass;
            }
        }
        // Revert item state changes
        self.sol
            .internal_ctl_affectors_restore(&mut saved_states, &mut reuse_eupdates);
        fleet_mass
    }
}
