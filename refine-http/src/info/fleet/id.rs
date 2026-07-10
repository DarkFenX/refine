use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFleetInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::FleetId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetInfoId {
    pub(in crate::info::fleet) fn from_core(core_fleet: &mut rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}
