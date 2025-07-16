use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id, check_fit_key},
    uad::{Uad, UadFighter},
};

impl UadFighter {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        check_fit_key(uad, self.get_fit_key())?;
        self.get_autocharges().consistency_check(uad)?;
        self.get_projs().consistency_check(uad)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let fighter_radius = uad.get_item_radius(uad.items.key_by_id(&self.get_item_id()).unwrap());
        for (projectee_key, prange) in self.get_projs().iter() {
            if let Some(prange) = prange {
                if prange.get_src_rad() != fighter_radius {
                    return Err(DebugError {});
                }
                if prange.get_tgt_rad() != uad.get_item_radius(projectee_key) {
                    return Err(DebugError {});
                }
            }
        }
        // If fighter has autocharges, make sure projections on them match
        if !self.get_autocharges().is_empty() {
            let fighter_projs = self.get_projs().iter().sorted().collect_vec();
            for &autocharge_key in self.get_autocharges().values() {
                let autocharge_projs = uad
                    .items
                    .get(autocharge_key)
                    .get_autocharge()
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
