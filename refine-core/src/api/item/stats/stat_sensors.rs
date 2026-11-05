use crate::{
    Count, PValue,
    stats::{
        ItemStats, StatInJam, StatSensors, StatTimeOptions,
        err::{StatItemError, StatProbingSizeError},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub fn get_stat_locks(&mut self) -> Result<Count, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_locks(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_lock_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_lock_range(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_scan_res(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_scan_res(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_sensors(&mut self) -> Result<StatSensors, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_sensors(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_dscan_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_dscan_range(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_probing_size(&mut self) -> Result<PValue, StatItemError<StatProbingSizeError>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_probing_size(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_incoming_jam(&mut self, time_options: StatTimeOptions) -> Result<StatInJam, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_incoming_jam(&mut sol.reuse.cseq_cache, &sol.u_data, item_uid, time_options)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
}
