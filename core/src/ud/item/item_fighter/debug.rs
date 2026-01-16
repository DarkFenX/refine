use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult},
    ud::{UData, UFighter},
};

impl UFighter {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        self.get_fit_uid().consistency_check(u_data)?;
        self.get_autocharges().consistency_check(u_data)?;
        self.get_projs().consistency_check(u_data)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let fighter_radius = self.get_radius();
        for (projectee_uid, proj_data) in self.get_projs().iter() {
            let proj_data = match proj_data {
                Some(proj_data) => proj_data,
                None => return Err(DebugError {}),
            };
            if proj_data.get_src_radius() != fighter_radius {
                return Err(DebugError {});
            }
            if proj_data.get_tgt_radius() != u_data.items.get(projectee_uid).get_direct_radius() {
                return Err(DebugError {});
            }
        }
        // If fighter has autocharges, make sure projections on them match
        if !self.get_autocharges().is_empty() {
            let fighter_projs = self.get_projs().iter().sorted().collect_vec();
            for autocharge_uid in self.get_autocharges().values() {
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
                    return Err(DebugError {});
                }
            }
        }
        Ok(())
    }
}
