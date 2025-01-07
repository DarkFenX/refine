use crate::sol::{svc::calc::debug::check_ctx_modifier, uad::SolUad, SolDebugResult};

use super::SolRevisionRegister;

impl SolRevisionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for ctx_modifier in self.item_add.iter() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        for ctx_modifier in self.item_remove.iter() {
            check_ctx_modifier(uad, ctx_modifier)?;
        }
        Ok(())
    }
}
