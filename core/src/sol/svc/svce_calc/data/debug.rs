use crate::{sol::SolView, util::DebugResult};

use super::SolSvcCalcData;

impl SolSvcCalcData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        self.attrs.debug_consistency_check(sol_view)?;
        self.mods.debug_consistency_check(sol_view)?;
        self.afee.debug_consistency_check(sol_view)?;
        self.buffs.debug_consistency_check(sol_view)?;
        self.deps.debug_consistency_check(sol_view)?;
        self.revs.debug_consistency_check(sol_view)?;
        Ok(())
    }
}
