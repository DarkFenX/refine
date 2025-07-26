use crate::{
    dbg::{DebugError, DebugResult, check_effect_key},
    ud::{UData, UProjEffect},
};

impl UProjEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for &effect_key in reffs.iter() {
                check_effect_key(u_data, effect_key)?;
            }
        }
        self.get_projs().consistency_check(u_data)?;
        // All projections are supposed to be without range on projected effect
        for (_projectee_key, prange) in self.get_projs().iter() {
            if prange.is_some() {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
