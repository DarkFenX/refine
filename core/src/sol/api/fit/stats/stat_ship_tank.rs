use crate::{
    def::AttrVal,
    err::FitShipStatError,
    misc::{DmgKinds, DpsProfile, Spool},
    sol::api::{FitMut, ItemMutCommon},
    svc::vast::{StatLayerEhp, StatLayerErps, StatLayerHp, StatLayerRps, StatTank},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_hp(&mut self) -> Result<StatTank<StatLayerHp>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_hp()?)
    }
    pub fn get_stat_ehp(
        &mut self,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<Option<StatTank<StatLayerEhp>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_ehp(incoming_dps)?)
    }
    pub fn get_stat_wc_ehp(&mut self) -> Result<Option<StatTank<StatLayerEhp>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_wc_ehp()?)
    }
    pub fn get_stat_rps(&mut self, spool: Option<Spool>) -> Result<StatTank<StatLayerRps>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_rps(spool)?)
    }
    pub fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        spool: Option<Spool>,
    ) -> Result<Option<StatTank<StatLayerErps>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_erps(incoming_dps, spool)?)
    }
    pub fn get_stat_resists(&mut self) -> Result<StatTank<DmgKinds<AttrVal>>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_resists()?)
    }
}
