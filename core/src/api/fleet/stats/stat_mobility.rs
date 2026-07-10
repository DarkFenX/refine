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
        // Collect IDs of fleet ships
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        let fit_uids = u_fleet.iter_fits();
        let mut ship_uids = Vec::with_capacity(fit_uids.len());
        for fit_uid in fit_uids {
            let u_fit = self.sol.u_data.fits.get(fit_uid);
            let Some(ship_uid) = u_fit.ship else {
                continue;
            };
            ship_uids.push(ship_uid);
        }
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
