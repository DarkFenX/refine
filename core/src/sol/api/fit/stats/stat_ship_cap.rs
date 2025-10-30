use crate::{
    def::AttrVal,
    sol::api::{FitMut, FitShipStatError, ItemMutCommon},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_cap(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap()?)
    }
    pub fn get_stat_cap_regen(&mut self, cap_perc: Option<AttrVal>) -> Result<Option<AttrVal>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_regen(cap_perc)?)
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
