use super::err::FitShipStatError;
use crate::{
    def::AttrVal,
    sol::api::{FitMut, ItemMutCommon},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_speed(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_speed()?)
    }
    pub fn get_stat_agility(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_agility()?)
    }
    pub fn get_stat_align_time(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_align_time()?)
    }
}
