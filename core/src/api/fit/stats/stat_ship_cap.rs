use crate::{
    api::{FitMut, FitShipStatError, ItemMutCommon},
    misc::{PValue, UnitInterval, Value},
    svc::vast::{StatCapSim, StatCapSimStagger, StatCapSrcKinds, StatTimeOptions},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_cap_amount(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_amount()?)
    }
    pub fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, FitShipStatError> {
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
    pub fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
