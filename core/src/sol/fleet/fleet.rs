use crate::{
    defs::{SolFitId, SolFleetId},
    util::StSet,
};

pub(in crate::sol) struct SolFleet {
    pub(in crate::sol) id: SolFleetId,
    fits: StSet<SolFitId>,
}
impl SolFleet {
    pub(in crate::sol) fn new(id: SolFleetId) -> Self {
        Self { id, fits: StSet::new() }
    }
    pub(in crate::sol) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &SolFitId> {
        self.fits.iter()
    }
    pub(in crate::sol) fn contains_fit(&self, fit: &SolFitId) -> bool {
        self.fits.contains(fit)
    }
    pub(in crate::sol) fn add_fit(&mut self, fit_id: SolFitId) {
        self.fits.insert(fit_id);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &SolFitId) {
        self.fits.remove(fit_id);
    }
}
