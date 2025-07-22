use crate::{
    def::FleetId,
    uad::UadFitKey,
    util::{GetId, Named, RSet},
};

#[derive(Clone)]
pub(crate) struct UadFleet {
    pub(crate) id: FleetId,
    fits: RSet<UadFitKey>,
}
impl UadFleet {
    pub(crate) fn new(id: FleetId) -> Self {
        Self { id, fits: RSet::new() }
    }
    pub(crate) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &UadFitKey> {
        self.fits.iter()
    }
    pub(crate) fn contains_fit(&self, fit: &UadFitKey) -> bool {
        self.fits.contains(fit)
    }
    pub(crate) fn add_fit(&mut self, fit_key: UadFitKey) {
        self.fits.insert(fit_key);
    }
    pub(crate) fn remove_fit(&mut self, fit_key: &UadFitKey) {
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
