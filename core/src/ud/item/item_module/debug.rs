use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult, check_fit_uid, check_item_uid},
    ud::{UData, UModule},
};

impl UModule {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_uid(u_data, self.get_fit_uid())?;
        if let Some(charge_uid) = self.get_charge_uid() {
            check_item_uid(u_data, charge_uid, false)?;
        }
        self.get_projs().consistency_check(u_data)?;
        // Radius of projector should match radius of ship, radius of projectee should match
        // projectee items
        let ship_radius = u_data.get_ship_radius_by_fit_uid(self.get_fit_uid());
        for (projectee_uid, proj_data) in self.get_projs().iter() {
            let proj_data = match proj_data {
                Some(proj_data) => proj_data,
                None => return Err(DebugError {}),
            };
            if proj_data.get_src_radius() != ship_radius {
                return Err(DebugError {});
            }
            if proj_data.get_tgt_radius() != u_data.items.get(projectee_uid).get_direct_radius() {
                return Err(DebugError {});
            }
        }
        // If module has a charge, make sure projections on them match
        if let Some(charge_uid) = self.get_charge_uid() {
            let module_projs = self.get_projs().iter().sorted().collect_vec();
            let charge_projs = u_data
                .items
                .get(charge_uid)
                .dc_charge()
                .unwrap()
                .get_projs()
                .iter()
                .sorted()
                .collect_vec();
            if module_projs != charge_projs {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
