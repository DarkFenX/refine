use crate::{
    def::AttrVal,
    sol::api::{FitMut, FitShipStatError, ItemMutCommon},
    svc::vast::{StatCapSim, StatCapSrcKinds},
    util::UnitInterval,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_cap_amount(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_amount()?)
    }
    pub fn get_stat_cap_balance(&mut self, src_kinds: StatCapSrcKinds) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_balance(src_kinds)?)
    }
    pub fn get_stat_cap_sim(&mut self, cap_perc: Option<UnitInterval>) -> Result<StatCapSim, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_sim(cap_perc)?)
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
