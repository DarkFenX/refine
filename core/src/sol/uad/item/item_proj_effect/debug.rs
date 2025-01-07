use crate::sol::{uad::SolUad, SolDebugResult};

use super::SolProjEffect;

impl SolProjEffect {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
