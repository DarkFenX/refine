use crate::util::HExecError;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolFleetId,
}
impl HFleetInfoId {
    pub(in crate::info::fleet) fn mk_info(
        core_sol: &rc::SolarSystem,
        fleet_id: &rc::SolFleetId,
    ) -> Result<Self, HExecError> {
        let core_fleet = match core_sol.get_fleet(fleet_id) {
            Ok(core_fleet) => core_fleet,
            Err(error) => match error {
                rc::err::GetFleetError::FleetNotFound(e) => return Err(HExecError::FleetNotFoundPrimary(e)),
            },
        };
        let info = Self { id: core_fleet.id };
        Ok(info)
    }
}
