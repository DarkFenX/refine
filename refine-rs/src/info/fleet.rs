use rc::Lender;

use crate::info::FleetInfoMode;

pub struct FleetInfo {
    pub id: rc::FleetId,
    pub extended: Option<FleetInfoExt>,
}

pub struct FleetInfoExt {
    pub fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfo {
    pub(crate) fn from_core(core_fleet: &mut rc::FleetMut, fleet_mode: FleetInfoMode) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
            extended: match fleet_mode {
                FleetInfoMode::Id => None,
                FleetInfoMode::Full => Some(FleetInfoExt {
                    fit_ids: core_fleet
                        .iter_fits_mut()
                        .map_into_iter(|core_fit| core_fit.get_fit_id())
                        .collect(),
                }),
            },
        }
    }
}
