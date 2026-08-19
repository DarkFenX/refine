use rc::Lender;

use crate::{FitId, FitInfo, FitInfoMode, FleetId, FleetInfoMode, ItemId, ItemInfoMode, shared::OvrdMapLight};

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
    pub fits: Vec<FitInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfo {
    pub(in crate::info) fn from_core(
        core_fleet: &mut rc::FleetMut,
        fleet_info_modes: &OvrdMapLight<FleetId, FleetInfoMode>,
        fit_info_modes: &OvrdMapLight<FitId, FitInfoMode>,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let fleet_id = core_fleet.get_fleet_id();
        let fleet_info_mode = fleet_info_modes.get(&fleet_id);
        Self {
            id: fleet_id,
            extended: match fleet_info_mode {
                FleetInfoMode::Id => None,
                FleetInfoMode::Full => Some(FleetInfoExt {
                    fits: core_fleet
                        .iter_fits_mut()
                        .map_into_iter(|mut core_fit| {
                            FitInfo::from_core(&mut core_fit, fit_info_modes, item_info_modes)
                        })
                        .collect(),
                }),
            },
        }
    }
}
