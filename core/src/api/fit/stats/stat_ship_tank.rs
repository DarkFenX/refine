use crate::{
    api::{FitMut, ItemMutCommon},
    def::AttrVal,
    err::FitShipStatError,
    misc::{DmgKinds, DpsProfile},
    svc::vast::{
        StatLayerEhp, StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatTank,
        StatTankRegen, StatTimeOptions,
    },
    util::UnitInterval,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_resists(&mut self) -> Result<StatTank<DmgKinds<AttrVal>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_resists()?)
    }
    pub fn get_stat_hp(&mut self) -> Result<StatTank<StatLayerHp>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_hp()?)
    }
    pub fn get_stat_ehp(
        &mut self,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_ehp(incoming_dps)?)
    }
    pub fn get_stat_wc_ehp(&mut self) -> Result<StatTank<Option<StatLayerEhp>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_wc_ehp()?)
    }
    pub fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_rps(time_options, shield_perc)?)
    }
    pub fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_erps(incoming_dps, time_options, shield_perc)?)
    }
}
