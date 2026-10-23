use itertools::Itertools;

use crate::{
    dbg::DebugResult,
    ud::{UData, UFighter},
};

impl UFighter {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            autocharges,
            projs,
            // Not checked
            count_override: _,
            physics: _,
            rearm_minion_override: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        autocharges.consistency_check(u_data)?;
        projs.consistency_check(u_data)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let fighter_radius = self.get_radius();
        for (projectee_uid, proj_data) in projs.iter() {
            let Some(proj_data) = proj_data else {
                return Err(Default::default());
            };
            if proj_data.get_src_radius() != fighter_radius {
                return Err(Default::default());
            }
            if proj_data.get_tgt_radius() != u_data.items.get(projectee_uid).get_direct_radius() {
                return Err(Default::default());
            }
        }
        // If fighter has autocharges, make sure projections on them match
        if !autocharges.is_empty() {
            let fighter_projs = projs.iter().sorted().collect_vec();
            for autocharge_uid in autocharges.values() {
                let autocharge_projs = u_data
                    .items
                    .get(autocharge_uid)
                    .dc_autocharge()
                    .unwrap()
                    .get_projs()
                    .iter()
                    .sorted()
                    .collect_vec();
                if fighter_projs != autocharge_projs {
                    return Err(Default::default());
                }
            }
        }
        Ok(())
    }
}
