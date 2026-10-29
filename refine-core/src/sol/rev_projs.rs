use crate::{ud::UItemId, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(crate) struct RevProjs {
    data: RMapRSet<UItemId, UItemId>,
}
impl RevProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(crate) fn reg_projectee(&mut self, projector_uid: UItemId, projectee_uid: UItemId) {
        self.data.add_entry(projectee_uid, projector_uid)
    }
    pub(crate) fn unreg_projectee(&mut self, projector_uid: &UItemId, projectee_uid: UItemId) {
        self.data.remove_entry(projectee_uid, projector_uid);
    }
    pub(crate) fn iter_projectors(&self, projectee_uid: &UItemId) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.get(projectee_uid).copied()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use itertools::Itertools;

    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl RevProjs {
        pub(in crate::sol) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            for (projectee_uid, projector_uids) in self.data.iter() {
                if u_data.items.try_get(*projectee_uid).is_none() {
                    return Err(Default::default());
                }
                // Check that projector item exists, and has projectee in its projections
                for projector_uid in projector_uids {
                    let Some(projector_u_item) = u_data.items.try_get(*projector_uid) else {
                        return Err(Default::default());
                    };
                    let Some(projector_projections) = projector_u_item.get_projs() else {
                        return Err(Default::default());
                    };
                    if !projector_projections.contains(projectee_uid) {
                        return Err(Default::default());
                    }
                }
            }
            // Check that all projections on items have corresponding entry in the tracker
            for (projector_uid, u_item) in u_data.items.iter() {
                if let Some(projections) = u_item.get_projs() {
                    for projectee_uid in projections.iter_projectees() {
                        if !self.data.get(&projectee_uid).contains(&projector_uid) {
                            return Err(Default::default());
                        }
                    }
                }
            }
            Ok(())
        }
    }
}
