use crate::{
    Count, FitMut, ItemMutCommon, PValue,
    stats::{
        StatInJam, StatSensors, StatTimeOptions,
        err::{StatFitShipError, StatProbingSizeError},
    },
};

impl<'s> FitMut<'s> {
    pub fn get_stat_locks(&mut self) -> Result<Count, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_locks()?)
    }
    pub fn get_stat_lock_range(&mut self) -> Result<PValue, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_lock_range()?)
    }
    pub fn get_stat_scan_res(&mut self) -> Result<PValue, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_scan_res()?)
    }
    pub fn get_stat_sensors(&mut self) -> Result<StatSensors, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_sensors()?)
    }
    pub fn get_stat_dscan_range(&mut self) -> Result<PValue, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_dscan_range()?)
    }
    pub fn get_stat_probing_size(&mut self) -> Result<PValue, StatFitShipError<StatProbingSizeError>> {
        Ok(self.get_ship_for_stats()?.get_stat_probing_size()?)
    }
    pub fn get_stat_incoming_jam(&mut self, time_options: StatTimeOptions) -> Result<StatInJam, StatFitShipError<!>> {
        Ok(self.get_ship_for_stats()?.get_stat_incoming_jam(time_options)?)
    }
}
