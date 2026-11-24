use crate::{Fit, FitId, sol::SolCore, ud::UFitId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats of a fit.
pub struct FitStats<'sol> {
    pub(in crate::api) sol: &'sol mut SolCore,
    pub(in crate::api) uid: UFitId,
}
impl<'sol> FitStats<'sol> {
    pub fn get_fit_id(&self) -> FitId {
        self.sol.u_data.fits.ext_id_by_int_id(self.uid)
    }
}
impl<'sol> Fit<'sol> {
    pub fn stats(&mut self) -> FitStats<'_> {
        self.sol.reuse.cseq_cache.clear();
        FitStats::new(self.sol, self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> FitStats<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UFitId) -> Self {
        Self { sol, uid }
    }
}
