use rc::Lender;

use crate::{FitId, FleetId, FleetInfoMode};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FleetInfo {
    pub id: FleetId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FleetInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FleetInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fit_ids: Vec<FitId>,
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
