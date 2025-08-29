use super::err::FitShipStatError;
use crate::{
    def::{AttrVal, Count},
    sol::api::{FitMut, ItemMutCommon},
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
}
