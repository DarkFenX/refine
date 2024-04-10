use std::collections::HashSet;

use crate::defs::{SsFitId, SsFleetId};

pub(in crate::ss) struct SsFleet {
    pub(in crate::ss) id: SsFleetId,
    pub(in crate::ss) fits: HashSet<SsFitId>,
}
impl SsFleet {
    pub(in crate::ss) fn new(id: SsFleetId) -> Self {
        Self {
            id,
            fits: HashSet::new(),
        }
    }
    pub(in crate::ss) fn add_fit(&mut self, fit_id: SsFitId) {
        self.fits.insert(fit_id);
    }
    pub(in crate::ss) fn remove_fit(&mut self, fit_id: &SsFitId) {
        self.fits.remove(fit_id);
    }
}
