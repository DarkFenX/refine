use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id},
    uad::{Uad, UadProjEffect},
};

impl UadProjEffect {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
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
