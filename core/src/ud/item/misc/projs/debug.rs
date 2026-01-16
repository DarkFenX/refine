use crate::{
    dbg::DebugResult,
    ud::{UData, item::misc::UProjs},
};

impl UProjs {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (projectee_uid, proj_data) in self.data.iter() {
            // Projectee is not necessarily loaded
            projectee_uid.consistency_check(u_data, false)?;
            if let Some(proj_data) = proj_data {
                proj_data.consistency_check()?;
            }
        }
        Ok(())
    }
}
