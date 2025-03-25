use crate::{
    sol::{FitId, FleetId},
    util::StSet,
};

#[derive(Clone)]
pub(in crate::sol) struct Fleet {
    pub(in crate::sol) id: FleetId,
    fits: StSet<FitId>,
}
impl Fleet {
    pub(in crate::sol) fn new(id: FleetId) -> Self {
        Self { id, fits: StSet::new() }
    }
    pub(in crate::sol) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &FitId> {
        self.fits.iter()
    }
    pub(in crate::sol) fn contains_fit(&self, fit: &FitId) -> bool {
        self.fits.contains(fit)
    }
    pub(in crate::sol) fn add_fit(&mut self, fit_id: FitId) {
        self.fits.insert(fit_id);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &FitId) {
        self.fits.remove(fit_id);
    }
}
