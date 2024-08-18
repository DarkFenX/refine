use crate::sol::{SolDebugResult, SolView};

use super::SolFighter;

impl SolFighter {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        self.get_autocharges().debug_consistency_check(sol_view)?;
        Ok(())
    }
}
