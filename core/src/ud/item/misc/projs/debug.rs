use crate::{
    dbg::{DebugResult, check_item_key},
    ud::{UData, item::misc::Projs},
};

impl Projs {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (projectee_key, proj_range) in self.data.iter() {
            // Projectee is not necessarily loaded
            check_item_key(u_data, *projectee_key, false)?;
            if let Some(proj_range) = proj_range {
                proj_range.consistency_check()?;
            }
        }
        Ok(())
    }
}
