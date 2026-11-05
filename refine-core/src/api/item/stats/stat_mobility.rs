use itertools::Itertools;

use crate::{
    CtlAffectors, FitId, PValue,
    api::AffectionDir,
    stats::{
        ItemStats, StatJump, StatJumpRange,
        err::{StatAgilityError, StatItemError, StatJumpError, StatMaxWarpRangeError, StatWarpSpeedError},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub fn get_stat_speed(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_speed(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_agility(&mut self) -> Result<PValue, StatItemError<StatAgilityError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_agility(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_align_time(&mut self) -> Result<PValue, StatItemError<StatAgilityError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_align_time(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_sig_radius(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_sig_radius(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_mass(&mut self, affectors: CtlAffectors) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = sol.internal_tx_savepoint();
        sol.internal_ctl_affectors_switch(
            item_uid,
            sol.u_data.r_data.get_attr_consts().mass,
            affectors,
            AffectionDir::Increase,
        );
        let result = sol
            .svc
            .get_stat_item_mass(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_warp_speed(&mut self) -> Result<PValue, StatItemError<StatWarpSpeedError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_warp_speed(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_max_warp_range(&mut self) -> Result<PValue, StatItemError<StatMaxWarpRangeError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_max_warp_range(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_jump(
        &mut self,
        range: StatJumpRange,
        passenger_fit_ids: &[FitId],
        passenger_fuel_affectors: CtlAffectors,
    ) -> Result<StatJump, StatItemError<StatJumpError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = sol.internal_tx_savepoint();
        let passenger_fit_uids = passenger_fit_ids
            .iter()
            .filter_map(|fit_id| sol.u_data.fits.int_id_by_ext_id(fit_id))
            .collect_vec();
        let passenger_ship_uids = sol.u_data.get_fits_ship_uids(passenger_fit_uids.iter().copied());
        // Work on item states according to request
        for &passenger_ship_uid in passenger_ship_uids.iter() {
            sol.internal_ctl_affectors_switch(
                passenger_ship_uid,
                sol.u_data.r_data.get_attr_consts().mass,
                passenger_fuel_affectors,
                AffectionDir::Increase,
            );
        }
        // Collect stats
        let result = sol
            .svc
            .get_stat_item_jump(&sol.u_data, item_uid, range, &passenger_fit_uids)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        // Revert item state changes
        sol.internal_tx_rollback(savepoint);
        result
    }
}
