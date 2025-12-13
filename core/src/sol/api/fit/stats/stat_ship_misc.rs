use crate::sol::api::{FitMut, FitShipStatError, ItemMutCommon};

impl<'a> FitMut<'a> {
    pub fn get_stat_can_warp(&mut self) -> Result<bool, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_can_warp()?)
    }
}
