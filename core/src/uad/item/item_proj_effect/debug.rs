use crate::{
    dbg::{DebugError, DebugResult},
    uad::{Uad, UadProjEffect},
};

impl UadProjEffect {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.get_projs().consistency_check(uad)?;
        // All projections are supposed to be without range on projected effect
        for (_projectee_key, prange) in self.get_projs().iter() {
            if prange.is_some() {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
