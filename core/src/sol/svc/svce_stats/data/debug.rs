use crate::sol::{SolDebugResult, SolView};

use super::SolSvcStatsData;

impl SolSvcStatsData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        self.cpu.debug_consistency_check(sol_view)?;
        Ok(())
    }
}
