use crate::{Sol, sol::SolCore};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats access to a solar system.
pub struct SolStats<'sol> {
    pub(in crate::api) sol: &'sol mut SolCore,
}
impl Sol {
    pub fn stats(&mut self) -> SolStats<'_> {
        let sol = self.inner_mut();
        sol.reuse.cseq_cache.clear();
        SolStats { sol }
    }
}
