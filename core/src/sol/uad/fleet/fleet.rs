use crate::{
    sol::{FitKey, FleetId},
    util::RSet,
};

#[derive(Clone)]
pub(in crate::sol) struct Fleet {
    pub(in crate::sol) id: FleetId,
    fits: RSet<FitKey>,
}
impl Fleet {
    pub(in crate::sol) fn new(id: FleetId) -> Self {
        Self { id, fits: RSet::new() }
    }
    pub(in crate::sol) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &FitKey> {
        self.fits.iter()
    }
    pub(in crate::sol) fn contains_fit(&self, fit: &FitKey) -> bool {
        self.fits.contains(fit)
    }
    pub(in crate::sol) fn add_fit(&mut self, fit_key: FitKey) {
        self.fits.insert(fit_key);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_key: &FitKey) {
        self.fits.remove(fit_key);
    }
}
