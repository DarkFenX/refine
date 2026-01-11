use crate::{
    api::{FitMut, ItemMutCommon},
    err::FitShipStatError,
    misc::DpsProfile,
    num::UnitInterval,
    svc::vast::{StatEhp, StatErps, StatHp, StatResists, StatRps, StatTimeOptions},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_resists(&mut self) -> Result<StatResists, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_resists()?)
    }
    pub fn get_stat_hp(&mut self) -> Result<StatHp, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_hp()?)
    }
    pub fn get_stat_ehp(&mut self, incoming_dps: Option<DpsProfile>) -> Result<StatEhp, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_ehp(incoming_dps)?)
    }
    pub fn get_stat_wc_ehp(&mut self) -> Result<StatEhp, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_wc_ehp()?)
    }
    pub fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_rps(time_options, shield_perc)?)
    }
    pub fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatErps, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_erps(incoming_dps, time_options, shield_perc)?)
    }
}
