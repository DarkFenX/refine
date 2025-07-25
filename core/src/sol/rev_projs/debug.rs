use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult},
    sol::rev_projs::RevProjs,
    ud::UData,
};

impl RevProjs {
    pub(in crate::sol) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (projectee_key, projector_keys) in self.data.iter() {
            if u_data.items.try_get(*projectee_key).is_none() {
                return Err(DebugError {});
            }
            // Check that projector item exists, and has projectee in its projections
            for projector_key in projector_keys {
                let projector_u_item = match u_data.items.try_get(*projector_key) {
                    Some(projector_u_item) => projector_u_item,
                    None => return Err(DebugError {}),
                };
                let projector_projections = match projector_u_item.get_projs() {
                    Some(projector_projections) => projector_projections,
                    None => return Err(DebugError {}),
                };
                if !projector_projections.contains(projectee_key) {
                    return Err(DebugError {});
                }
            }
        }
        // Check that all projections on items have corresponding entry in the tracker
        for (projector_key, u_item) in u_data.items.iter() {
            if let Some(projections) = u_item.get_projs() {
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
