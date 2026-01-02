use crate::{
    api::{FitMut, FitShipStatError, ItemMutCommon},
    def::AttrVal,
    svc::vast::{StatCapSim, StatCapSimStagger, StatCapSrcKinds, StatTimeOptions},
    util::UnitInterval,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_cap_amount(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_amount()?)
    }
    pub fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<AttrVal, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_cap_balance(src_kinds, time_options)?)
    }
    pub fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        reload_optionals: Option<bool>,
        stagger: StatCapSimStagger,
    ) -> Result<StatCapSim, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_cap_sim(cap_perc, reload_optionals, stagger)?)
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
