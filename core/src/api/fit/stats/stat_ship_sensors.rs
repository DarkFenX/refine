use crate::{
    api::{FitMut, FitShipStatError, ItemMutCommon},
    num::{Count, PValue},
    svc::vast::{StatInJam, StatSensors},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_locks(&mut self) -> Result<Count, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_locks()?)
    }
    pub fn get_stat_lock_range(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_lock_range()?)
    }
    pub fn get_stat_scan_res(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_scan_res()?)
    }
    pub fn get_stat_sensors(&mut self) -> Result<StatSensors, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_sensors()?)
    }
    pub fn get_stat_dscan_range(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_dscan_range()?)
    }
    pub fn get_stat_probing_size(&mut self) -> Result<Option<PValue>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_probing_size()?)
    }
    pub fn get_stat_incoming_jam(&mut self) -> Result<StatInJam, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_incoming_jam()?)
    }
}
