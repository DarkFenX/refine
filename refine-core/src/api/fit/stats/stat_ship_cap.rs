use crate::{
    api::{FitMut, FitShipAppliedStatError, FitShipStatError, ItemMutCommon},
    misc::OptionalReload,
    num::{PValue, UnitInterval, Value},
    svc::vast::{StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatTimeOptions},
    ud::ItemId,
};

impl<'s> FitMut<'s> {
    pub fn get_stat_cap_amount(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_amount()?)
    }
    pub fn get_stat_cap_balance(
        &mut self,
        src_kinds: &StatCapBlcSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, FitShipAppliedStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_cap_balance(src_kinds, time_options)?)
    }
    pub fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: StatCapSimStagger,
        nosf_projectee_item_id: Option<&ItemId>,
    ) -> Result<StatCapSim, FitShipAppliedStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_cap_sim(cap_perc, optional_reloads, stagger, nosf_projectee_item_id)?)
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
