use crate::{
    dbg::{DebugResult, check_item_uid},
    ud::{UData, item::misc::UProjs},
};

impl UProjs {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (projectee_uid, proj_data) in self.data.iter() {
            // Projectee is not necessarily loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            if let Some(proj_data) = proj_data {
                proj_data.consistency_check()?;
            }
        }
        Ok(())
    }
}
