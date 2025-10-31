use crate::{
    def::AttrVal,
    sol::api::{FitMut, FitShipStatError, ItemMutCommon},
    svc::vast::StatCapSrcKinds,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_cap_amount(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_amount()?)
    }
    pub fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapSrcKinds,
        regen_perc: Option<AttrVal>,
    ) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_cap_balance(src_kinds, regen_perc)?)
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<AttrVal, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_neut_resist()?)
    }
}
