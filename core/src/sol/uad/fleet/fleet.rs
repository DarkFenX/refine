use crate::{
    sol::{FitKey, FleetId},
    util::{GetId, Named, RSet},
};

#[derive(Clone)]
pub(in crate::sol) struct UadFleet {
    pub(in crate::sol) id: FleetId,
    fits: RSet<FitKey>,
}
impl UadFleet {
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
impl Named for UadFleet {
    fn get_name() -> &'static str {
        "Fleet"
    }
}
impl GetId<FleetId> for UadFleet {
    fn get_id(&self) -> FleetId {
        self.id
    }
}
