use super::RevisionRegister;
use crate::{dbg::DebugResult, svc::calc::debug::check_cmod, ud::UData};

impl RevisionRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for cmod in self.item_add.keys() {
            check_cmod(u_data, cmod)?;
        }
        for cmod in self.item_remove.keys() {
            check_cmod(u_data, cmod)?;
        }
        Ok(())
    }
}
