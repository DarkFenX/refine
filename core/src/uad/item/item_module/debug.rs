use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    uad::{Uad, UadModule},
};

impl UadModule {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        check_fit_key(uad, self.get_fit_key())?;
        if let Some(charge_key) = self.get_charge_key() {
            check_item_key(uad, charge_key, false)?;
        }
        self.get_projs().consistency_check(uad)?;
        // Radius of projector should match radius of ship, radius of projectee should match
        // projectee items
        let ship_radius = uad.get_ship_radius_by_fit_key(self.get_fit_key());
        for (projectee_key, prange) in self.get_projs().iter() {
            if let Some(prange) = prange {
                if prange.get_src_rad() != ship_radius {
                    return Err(DebugError {});
                }
                if prange.get_tgt_rad() != uad.get_item_radius(projectee_key) {
                    return Err(DebugError {});
                }
            }
        }
        // If module has a charge, make sure projections on them match
        if let Some(charge_key) = self.get_charge_key() {
            let module_projs = self.get_projs().iter().sorted().collect_vec();
            let charge_projs = uad
                .items
                .get(charge_key)
                .get_charge()
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
