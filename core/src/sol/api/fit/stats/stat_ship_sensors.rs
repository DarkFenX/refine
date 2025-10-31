use crate::{
    def::{AttrVal, Count},
    sol::api::{FitMut, FitShipStatError, ItemMutCommon},
    svc::vast::StatSensor,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_locks(&mut self) -> Result<Count, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_locks()?)
    }
    pub fn get_stat_lock_range(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_lock_range()?)
    }
    pub fn get_stat_scan_res(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_scan_res()?)
    }
    pub fn get_stat_sensor(&mut self) -> Result<StatSensor, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_sensor()?)
    }
    pub fn get_stat_dscan_range(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_dscan_range()?)
    }
    pub fn get_stat_probing_size(&mut self) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_probing_size()?)
    }
    pub fn get_stat_jam_chance(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_jam_chance()?)
    }
}
