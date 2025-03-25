use crate::sol::{debug::DebugResult, uad::Uad};

use super::Calc;

impl Calc {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
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
