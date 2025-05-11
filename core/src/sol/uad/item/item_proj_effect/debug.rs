use super::UadProjEffect;
use crate::sol::{debug::DebugResult, uad::Uad};

impl UadProjEffect {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
