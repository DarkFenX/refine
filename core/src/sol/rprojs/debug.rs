use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult},
    sol::rprojs::RProjs,
    uad::Uad,
};

impl RProjs {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (projectee_key, projector_keys) in self.data.iter() {
            if uad.items.try_get(*projectee_key).is_none() {
                return Err(DebugError {});
            }
            // Check that projector item exists, and has projectee in its projections
            for projector_key in projector_keys {
                let projector_uad_item = match uad.items.try_get(*projector_key) {
                    Some(projector_uuad_item) => projector_uuad_item,
                    None => return Err(DebugError {}),
                };
                let projector_projections = match projector_uad_item.get_projs() {
                    Some(projector_projections) => projector_projections,
                    None => return Err(DebugError {}),
                };
                if !projector_projections.contains(projectee_key) {
                    return Err(DebugError {});
                }
            }
        }
        // Check that all projections on items have corresponding entry in the tracker
        for (projector_key, uad_item) in uad.items.iter() {
            if let Some(projections) = uad_item.get_projs() {
                for projectee_key in projections.iter_projectees() {
                    if !self.data.get(&projectee_key).contains(&projector_key) {
                        return Err(DebugError {});
                    }
                }
            }
        }
        Ok(())
    }
}
