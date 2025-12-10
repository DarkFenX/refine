use crate::{
    def::FleetId,
    ud::UFitKey,
    util::{GetId, Named, RSet},
};

#[derive(Clone)]
pub(crate) struct UFleet {
    pub(crate) id: FleetId,
    fits: RSet<UFitKey>,
}
impl UFleet {
    pub(crate) fn new(id: FleetId) -> Self {
        Self { id, fits: RSet::new() }
    }
    pub(crate) fn iter_fits(&self) -> impl ExactSizeIterator<Item = UFitKey> {
        self.fits.iter().copied()
    }
    pub(crate) fn contains_fit(&self, fit: &UFitKey) -> bool {
        self.fits.contains(fit)
    }
    pub(crate) fn add_fit(&mut self, fit_key: UFitKey) {
        self.fits.insert(fit_key);
    }
    pub(crate) fn remove_fit(&mut self, fit_key: &UFitKey) {
        self.fits.remove(fit_key);
    }
}
impl Named for UFleet {
    fn get_name() -> &'static str {
        "UFleet"
    }
}
impl GetId<FleetId> for UFleet {
    fn get_id(&self) -> FleetId {
        self.id
    }
}
