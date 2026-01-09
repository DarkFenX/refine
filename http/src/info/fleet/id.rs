use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFleetInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::FleetId,
}
impl From<&mut rc::FleetMut<'_>> for HFleetInfoId {
    fn from(core_fleet: &mut rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}
