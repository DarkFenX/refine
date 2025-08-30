use super::err::FitShipStatError;
use crate::{
    def::AttrVal,
    sol::api::{FitMut, ItemMutCommon},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_speed(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_speed()?)
    }
    pub fn get_stat_agility(&mut self) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_agility()?)
    }
    pub fn get_stat_align_time(&mut self) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_align_time()?)
    }
    pub fn get_stat_sig_radius(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_sig_radius()?)
    }
    pub fn get_stat_mass(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_mass()?)
    }
    pub fn get_stat_warp_speed(&mut self) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_warp_speed()?)
    }
    pub fn get_stat_max_warp_range(&mut self) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_max_warp_range()?)
    }
}
