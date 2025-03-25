use crate::util::HExecError;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::FleetId,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<rc::FitId>,
}
impl HFleetInfoFull {
    pub(in crate::info::fleet) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<Self, HExecError> {
        let core_fleet = match core_sol.get_fleet(fleet_id) {
            Ok(core_fleet) => core_fleet,
            Err(error) => match error {
                rc::err::GetFleetError::FleetNotFound(e) => return Err(HExecError::FleetNotFoundPrimary(e)),
            },
        };
        let fleet = Self {
            id: *fleet_id,
            fits: core_fleet.fits.clone(),
        };
        Ok(fleet)
    }
}
