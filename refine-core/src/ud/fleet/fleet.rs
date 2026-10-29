use crate::{
    ud::{FleetId, UFitId},
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
    #[cfg(feature = "dev")]
    pub(crate) fn contains_fit(&self, fit: &UFitId) -> bool {
        self.fits.contains(fit)
    }
    pub(crate) fn add_fit(&mut self, fit_uid: UFitId) {
        self.fits.insert(fit_uid);
    }
    pub(crate) fn remove_fit(&mut self, fit_uid: &UFitId) {
        self.fits.remove(fit_uid);
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl UFleet {
        pub(in crate::ud) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self { id, fits } = self;
            let Some(fleet_uid) = u_data.fleets.int_id_by_ext_id(id) else {
                return Err(Default::default());
            };
            // Every fit referenced by the fleet should exist, and refer back to the fleet
            for &fit_uid in fits.iter() {
                let Some(fit) = u_data.fits.try_get(fit_uid) else {
                    return Err(Default::default());
                };
                if fit.fleet != Some(fleet_uid) {
                    return Err(Default::default());
                }
            }
            Ok(())
        }
    }
}
