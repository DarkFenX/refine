use crate::{
    api::{CtlAffectors, FitMut, FitShipStatError, ItemMutCommon},
    num::PValue,
    svc::vast::{StatJump, StatJumpRange},
    ud::FitId,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_speed(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_speed()?)
    }
    pub fn get_stat_agility(&mut self) -> Result<Option<PValue>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_agility()?)
    }
    pub fn get_stat_align_time(&mut self) -> Result<Option<PValue>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_align_time()?)
    }
    pub fn get_stat_sig_radius(&mut self) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_sig_radius()?)
    }
    pub fn get_stat_mass(&mut self, affectors: CtlAffectors) -> Result<PValue, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_mass(affectors)?)
    }
    pub fn get_stat_warp_speed(&mut self) -> Result<Option<PValue>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_warp_speed()?)
    }
    pub fn get_stat_max_warp_range(&mut self) -> Result<Option<PValue>, FitShipStatError> {
        Ok(self.get_ship_for_stats()?.get_stat_max_warp_range()?)
    }
    pub fn get_stat_jump(
        &mut self,
        range: StatJumpRange,
        passenger_fit_ids: &[FitId],
        passenger_fuel_affectors: CtlAffectors,
    ) -> Result<Option<StatJump>, FitShipStatError> {
        Ok(self
            .get_ship_for_stats()?
            .get_stat_jump(range, passenger_fit_ids, passenger_fuel_affectors)?)
    }
}
