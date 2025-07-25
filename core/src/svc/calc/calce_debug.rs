use super::Calc;
use crate::{dbg::DebugResult, ud::UData};

impl Calc {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.attrs.consistency_check(u_data)?;
        self.std.consistency_check(u_data)?;
        self.buffs.consistency_check(u_data)?;
        self.deps.consistency_check(u_data)?;
        self.revs.consistency_check(u_data)?;
        self.rah.consistency_check(u_data)?;
        Ok(())
    }
}
