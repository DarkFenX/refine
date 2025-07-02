use super::RevisionRegister;
use crate::{dbg::DebugResult, svc::calc::debug::check_cmod, uad::Uad};

impl RevisionRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for cmod in self.item_add.keys() {
            check_cmod(uad, cmod)?;
        }
        for cmod in self.item_remove.keys() {
            check_cmod(uad, cmod)?;
        }
        Ok(())
    }
}
