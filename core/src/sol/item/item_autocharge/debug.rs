use crate::sol::{SolDebugResult, SolView};

use super::SolAutocharge;

impl SolAutocharge {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        self.get_projs().debug_consistency_check(sol_view)?;
        Ok(())
    }
}
