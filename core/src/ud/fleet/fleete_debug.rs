use crate::{
    dbg::DebugResult,
    ud::{UData, fleet::UFleet},
};

impl UFleet {
    pub(in crate::ud) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Some(fleet_uid) = u_data.fleets.int_id_by_ext_id(&self.id) else {
            return Err(Default::default());
        };
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_uid in self.iter_fits() {
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
