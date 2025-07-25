use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id},
    ud::{UData, UProjEffect},
};

impl UProjEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(u_data, a_effect_id)?;
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
