#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FleetId,
}
impl From<&mut rc::FleetMut<'_>> for HFleetInfoId {
    fn from(core_fleet: &mut rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}
