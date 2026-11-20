use crate::{
    PValue,
    api::AffectionDir,
    stats::{FleetStats, StatCtlAffectors},
    util::RMap,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FleetStats<'s> {
    pub fn get_stat_mass(&mut self, increasers: StatCtlAffectors) -> PValue {
        let mut fleet_mass = PValue::ZERO;
        let ship_uids = self.sol.u_data.get_fleet_ship_uids(self.uid);
        // Work on item states according to request
        let mut new_states = RMap::new();
        for &ship_uid in ship_uids.iter() {
            self.sol.internal_ctl_affectors_find(
                &mut new_states,
                ship_uid,
                self.sol.u_data.r_data.get_attr_consts().mass,
                increasers,
                AffectionDir::Increase,
            );
        }
        let savepoint = self.sol.internal_ctl_affectors_switch(new_states);
        // Collect stats
        for ship_uid in ship_uids.into_iter() {
            if let Ok(ship_mass) = self.sol.svc.get_stat_item_mass(&self.sol.u_data, ship_uid) {
                fleet_mass += ship_mass;
            }
        }
        // Revert item state changes
        if let Some(savepoint) = savepoint {
            self.sol.internal_tx_rollback(savepoint);
        }
        fleet_mass
    }
}
