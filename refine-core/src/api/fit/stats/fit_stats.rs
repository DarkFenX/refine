use crate::{Fit, FitId, sol::SolCore, ud::UFitId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats of a fit.
pub struct FitStats<'s> {
    pub(in crate::api) sol: &'s mut SolCore,
    pub(in crate::api) uid: UFitId,
}
impl<'s> FitStats<'s> {
    pub fn get_fit_id(&self) -> FitId {
        self.sol.u_data.fits.ext_id_by_int_id(self.uid)
    }
}
impl<'s> Fit<'s> {
    pub fn stats(&mut self) -> FitStats<'_> {
        self.sol.reuse.cseq_cache.clear();
        FitStats::new(self.sol, self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FitStats<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UFitId) -> Self {
        Self { sol, uid }
    }
}
