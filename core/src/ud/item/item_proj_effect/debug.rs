use crate::{
    dbg::{DebugError, DebugResult},
    ud::{UData, UProjEffect},
};

impl UProjEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        self.get_projs().consistency_check(u_data)?;
        // All projections are supposed to be without range on projected effect
        for (_projectee_key, proj_data) in self.get_projs().iter() {
            if proj_data.is_some() {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
