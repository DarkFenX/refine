use itertools::Itertools;

use crate::{
    dbg::DebugResult,
    ud::{UData, UModule},
};

impl UModule {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            charge_uid,
            projs,
            // Not checked
            rack: _,
            pos: _,
            spool_override: _,
            optional_reload_override: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        if let Some(charge_uid) = charge_uid {
            charge_uid.consistency_check(u_data, false)?;
        }
        projs.consistency_check(u_data)?;
        // Radius of projector should match radius of ship, radius of projectee should match
        // projectee items
        let ship_radius = u_data.get_fit_ship_radius(*fit_uid);
        for (projectee_uid, proj_data) in projs.iter() {
            let Some(proj_data) = proj_data else {
                return Err(Default::default());
            };
            if proj_data.get_src_radius() != ship_radius {
                return Err(Default::default());
            }
            if proj_data.get_tgt_radius() != u_data.items.get(projectee_uid).get_direct_radius() {
                return Err(Default::default());
            }
        }
        // If module has a charge, make sure projections on them match
        if let Some(charge_uid) = *charge_uid {
            let module_projs = projs.iter().sorted().collect_vec();
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
                return Err(Default::default());
            }
        }
        Ok(())
    }
}
