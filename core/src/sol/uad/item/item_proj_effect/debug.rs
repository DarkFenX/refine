use crate::sol::{debug::DebugResult, uad::Uad};

use super::ProjEffect;

impl ProjEffect {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
