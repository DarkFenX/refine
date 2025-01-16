use crate::sol::{debug::check_fit, uad::SolUad, SolDebugResult};

use super::SolRig;

impl SolRig {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        check_fit(uad, &self.get_fit_id())?;
        Ok(())
    }
}
