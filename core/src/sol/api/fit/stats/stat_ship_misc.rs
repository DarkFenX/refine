use crate::sol::api::{FitMut, FitShipStatError, ItemMutCommon};

impl<'a> FitMut<'a> {
    pub fn get_stat_can_warp(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_warp()?)
    }
    pub fn get_stat_can_gate_jump(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_gate_jump()?)
    }
    pub fn get_stat_can_drive_jump(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_drive_jump()?)
    }
    pub fn get_stat_can_dock(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_dock()?)
    }
    pub fn get_stat_can_tether(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_tether()?)
    }
}
