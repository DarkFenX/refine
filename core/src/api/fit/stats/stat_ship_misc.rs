use crate::api::{FitMut, FitShipStatError, ItemMutCommon};

impl<'a> FitMut<'a> {
    pub fn get_stat_can_warp(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_warp()?)
    }
    pub fn get_stat_can_jump_gate(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_jump_gate()?)
    }
    pub fn get_stat_can_jump_drive(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_jump_drive()?)
    }
    pub fn get_stat_can_dock_station(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_dock_station()?)
    }
    pub fn get_stat_can_dock_citadel(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_dock_citadel()?)
    }
    pub fn get_stat_can_tether(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_tether()?)
    }
}
