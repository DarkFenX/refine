use super::RevisionRegister;
use crate::{dbg::DebugResult, ud::UData};

impl RevisionRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for cmod in self.item_add.keys() {
            cmod.consistency_check(u_data)?;
        }
        for cmod in self.item_remove.keys() {
            cmod.consistency_check(u_data)?;
        }
        Ok(())
    }
}
