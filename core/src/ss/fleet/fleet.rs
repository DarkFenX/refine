use crate::{
    defs::{SsFitId, SsFleetId},
    util::StSet,
};

pub(in crate::ss) struct SsFleet {
    pub(in crate::ss) id: SsFleetId,
    fits: StSet<SsFitId>,
}
impl SsFleet {
    pub(in crate::ss) fn new(id: SsFleetId) -> Self {
        Self { id, fits: StSet::new() }
    }
    pub(in crate::ss) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &SsFitId> {
        self.fits.iter()
    }
    pub(in crate::ss) fn contains_fit(&self, fit: &SsFitId) -> bool {
        self.fits.contains(fit)
    }
    pub(in crate::ss) fn add_fit(&mut self, fit_id: SsFitId) {
        self.fits.insert(fit_id);
    }
    pub(in crate::ss) fn remove_fit(&mut self, fit_id: &SsFitId) {
        self.fits.remove(fit_id);
    }
}
