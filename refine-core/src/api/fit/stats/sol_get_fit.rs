use crate::{
    FitId,
    err::FitGetError,
    stats::{FitStats, SolStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SolStats<'s> {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<FitStats<'_>, FitGetError> {
        let fit_uid = self.sol.internal_get_sol_fit_uid(fit_id)?;
        Ok(FitStats::new(self.sol, fit_uid))
    }
}
