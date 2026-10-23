use crate::{
    dbg::DebugResult,
    ud::{UData, UDrone},
};

impl UDrone {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            projs,
            // Not checked
            physics: _,
            npc_prop_override: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        projs.consistency_check(u_data)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let drone_radius = self.get_radius();
        for (projectee_uid, proj_data) in projs.iter() {
            let Some(proj_data) = proj_data else {
                return Err(Default::default());
            };
            if proj_data.get_src_radius() != drone_radius {
                return Err(Default::default());
            }
            if proj_data.get_tgt_radius() != u_data.items.get(projectee_uid).get_direct_radius() {
                return Err(Default::default());
            }
        }
        Ok(())
    }
}
