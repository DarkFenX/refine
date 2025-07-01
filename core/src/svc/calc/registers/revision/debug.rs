use super::RevisionRegister;
use crate::{dbg::DebugResult, svc::calc::debug::check_ctx_modifier, uad::Uad};

impl RevisionRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for ctx_modifier in self.item_add.keys() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        for ctx_modifier in self.item_remove.keys() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        Ok(())
    }
}
