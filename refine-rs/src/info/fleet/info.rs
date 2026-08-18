use rc::Lender;

use crate::{FitId, FleetId, FleetInfoMode, shared::OverridableMap};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct FleetInfo {
    pub id: FleetId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FleetInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct FleetInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fit_ids: Vec<FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfo {
    pub(in crate::info) fn from_core(
        core_fleet: &mut rc::FleetMut,
        fleet_info_modes: &OverridableMap<FleetId, FleetInfoMode>,
    ) -> Self {
        let fleet_id = core_fleet.get_fleet_id();
        let fleet_info_mode = fleet_info_modes.get(&fleet_id);
        Self {
            id: fleet_id,
            extended: match fleet_info_mode {
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
