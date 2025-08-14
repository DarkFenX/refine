use crate::{
    dbg::{DebugError, DebugResult, check_fit_key},
    ud::{UData, UDrone},
};

impl UDrone {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_key(u_data, self.get_fit_key())?;
        self.get_projs().consistency_check(u_data)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let drone_radius = u_data.get_item_radius(u_data.items.key_by_id(&self.get_item_id()).unwrap());
        for (projectee_key, proj_data) in self.get_projs().iter() {
            let proj_data = match proj_data {
                Some(proj_data) => proj_data,
                None => return Err(DebugError {}),
            };
            if proj_data.get_src_rad() != drone_radius {
                return Err(DebugError {});
            }
            if proj_data.get_tgt_rad() != u_data.get_item_radius(projectee_key) {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
