use crate::sol::{debug::DebugResult, uad::Uad};

use super::Calc;

impl Calc {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.attrs.consistency_check(uad)?;
        self.std.consistency_check(uad)?;
        self.buffs.consistency_check(uad)?;
        self.deps.consistency_check(uad)?;
        self.revs.consistency_check(uad)?;
        self.projs.consistency_check(uad)?;
        self.rah.consistency_check(uad)?;
        Ok(())
    }
}
