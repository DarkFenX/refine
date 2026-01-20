use rc::Lender;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFleetInfoFull {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::FleetId,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fits: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetInfoFull {
    pub(in crate::info::fleet) fn from_core(core_fleet: &mut rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
            fits: core_fleet
                .iter_fits_mut()
                .map_into_iter(|core_fit| core_fit.get_fit_id())
                .collect(),
        }
    }
}
