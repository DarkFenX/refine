use crate::{
    def::AttrVal,
    err::FitShipStatError,
    misc::{DmgKinds, DpsProfile, Spool},
    sol::api::{FitMut, ItemMutCommon},
    svc::vast::{
        StatLayerEhp, StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatTank,
        StatTankRegen,
    },
    util::UnitInterval,
};

impl<'a> FitMut<'a> {
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
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_rps(shield_perc, spool)?)
    }
    pub fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_erps(incoming_dps, shield_perc, spool)?)
    }
    pub fn get_stat_resists(&mut self) -> Result<StatTank<DmgKinds<AttrVal>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_resists()?)
    }
}
