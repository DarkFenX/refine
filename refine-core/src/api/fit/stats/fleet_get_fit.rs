use crate::{
    FitId,
    err::FleetFitGetError,
    stats::{FitStats, FleetStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FleetStats<'s> {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<FitStats<'_>, FleetFitGetError> {
        let fit_uid = self.sol.internal_get_fleet_fit_uid(self.uid, fit_id)?;
        Ok(FitStats::new(self.sol, fit_uid))
    }
}
