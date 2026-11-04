use crate::{CtlAffectors, Fleet, PValue, api::AffectionDir};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_stat_mass(&mut self, ctl_affectors: CtlAffectors) -> PValue {
        let mut fleet_mass = PValue::ZERO;
        let savepoint = self.core.internal_tx_savepoint();
        let ship_uids = self.core.u_data.get_fleet_ship_uids(self.uid);
        // Work on item states according to request
        for &ship_uid in ship_uids.iter() {
            self.core.internal_ctl_affectors_switch(
                ship_uid,
                self.core.u_data.r_data.get_attr_consts().mass,
                ctl_affectors,
                AffectionDir::Increase,
            );
        }
        // Collect stats
        for ship_uid in ship_uids.into_iter() {
            if let Ok(ship_mass) = self.core.svc.get_stat_item_mass(&self.core.u_data, ship_uid) {
                fleet_mass += ship_mass;
            }
        }
        // Revert item state changes
        self.core.internal_tx_rollback(savepoint);
        fleet_mass
    }
}
