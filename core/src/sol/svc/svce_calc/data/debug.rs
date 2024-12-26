use crate::sol::{SolDebugResult, SolView};

use super::SolSvcCalcData;

impl SolSvcCalcData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        self.attrs.debug_consistency_check(sol_view)?;
        self.std.debug_consistency_check(sol_view)?;
        self.buffs.debug_consistency_check(sol_view)?;
        self.deps.debug_consistency_check(sol_view)?;
        self.revs.debug_consistency_check(sol_view)?;
        self.projs.debug_consistency_check(sol_view)?;
        self.rah.debug_consistency_check(sol_view)?;
        Ok(())
    }
}
