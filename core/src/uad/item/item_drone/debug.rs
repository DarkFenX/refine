use crate::{
    dbg::{DebugError, DebugResult, check_fit_key},
    uad::{Uad, UadDrone},
};

impl UadDrone {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        self.get_projs().consistency_check(uad)?;
        // Radius of projector should match radius of drone, radius of projectee should match
        // projectee items
        let drone_radius = uad.get_item_radius(uad.items.key_by_id(&self.get_item_id()).unwrap());
        for (projectee_key, prange) in self.get_projs().iter() {
            if let Some(prange) = prange {
                if prange.src_rad != drone_radius {
                    return Err(DebugError {});
                }
                if prange.tgt_rad != uad.get_item_radius(projectee_key) {
                    return Err(DebugError {});
                }
            }
        }
        Ok(())
    }
}
