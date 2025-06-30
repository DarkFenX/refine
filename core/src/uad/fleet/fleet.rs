use crate::{
    def::{FitKey, FleetId},
    util::{GetId, Named, RSet},
};

#[derive(Clone)]
pub(crate) struct UadFleet {
    pub(crate) id: FleetId,
    fits: RSet<FitKey>,
}
impl UadFleet {
    pub(crate) fn new(id: FleetId) -> Self {
        Self { id, fits: RSet::new() }
    }
    pub(crate) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &FitKey> {
        self.fits.iter()
    }
    pub(crate) fn contains_fit(&self, fit: &FitKey) -> bool {
        self.fits.contains(fit)
    }
    pub(crate) fn add_fit(&mut self, fit_key: FitKey) {
        self.fits.insert(fit_key);
    }
    pub(crate) fn remove_fit(&mut self, fit_key: &FitKey) {
        self.fits.remove(fit_key);
    }
}
impl Named for UadFleet {
    fn get_name() -> &'static str {
        "UadFleet"
    }
}
impl GetId<FleetId> for UadFleet {
    fn get_id(&self) -> FleetId {
        self.id
    }
}
