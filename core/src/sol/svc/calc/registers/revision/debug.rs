use crate::sol::{debug::DebugResult, svc::calc::debug::check_ctx_modifier, uad::Uad};

use super::RevisionRegister;

impl RevisionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for ctx_modifier in self.item_add.iter() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        for ctx_modifier in self.item_remove.iter() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        Ok(())
    }
}
