use crate::sol::{uad::SolUad, SolDebugResult};

use super::SolSvcCalcData;

impl SolSvcCalcData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        self.attrs.debug_consistency_check(uad)?;
        self.std.debug_consistency_check(uad)?;
        self.buffs.debug_consistency_check(uad)?;
        self.deps.debug_consistency_check(uad)?;
        self.revs.debug_consistency_check(uad)?;
        self.projs.debug_consistency_check(uad)?;
        self.rah.debug_consistency_check(uad)?;
        Ok(())
    }
}
