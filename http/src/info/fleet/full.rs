use rc::Lender;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FleetId,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fits: Vec<rc::FitId>,
}
impl From<&mut rc::FleetMut<'_>> for HFleetInfoFull {
    fn from(core_fleet: &mut rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
            fits: core_fleet
                .iter_fits_mut()
                .map_into_iter(|core_fit| core_fit.get_fit_id())
                .collect(),
        }
    }
}
