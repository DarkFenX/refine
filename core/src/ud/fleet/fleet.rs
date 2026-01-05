use crate::{
    api::FleetId,
    ud::UFitId,
    util::{LibGetId, LibNamed, RSet},
};

#[derive(Clone)]
pub(crate) struct UFleet {
    pub(crate) id: FleetId,
    fits: RSet<UFitId>,
}
impl UFleet {
    pub(crate) fn new(id: FleetId) -> Self {
        Self { id, fits: RSet::new() }
    }
    pub(crate) fn iter_fits(&self) -> impl ExactSizeIterator<Item = UFitId> {
        self.fits.iter().copied()
    }
    pub(crate) fn contains_fit(&self, fit: &UFitId) -> bool {
        self.fits.contains(fit)
    }
    pub(crate) fn add_fit(&mut self, fit_key: UFitId) {
        self.fits.insert(fit_key);
    }
    pub(crate) fn remove_fit(&mut self, fit_key: &UFitId) {
        self.fits.remove(fit_key);
    }
}
impl LibNamed for UFleet {
    fn lib_get_name() -> &'static str {
        "UFleet"
    }
}
impl LibGetId<FleetId> for UFleet {
    fn lib_get_id(&self) -> FleetId {
        self.id
    }
}
