use crate::util::HExecResult;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolFleetId,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<rc::SolFitId>,
}
impl HFleetInfoFull {
    pub(in crate::info::fleet) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::SolFleetId,
    ) -> HExecResult<Self> {
        let core_fleet = core_sol.get_fleet(fleet_id)?;
        let fleet = Self {
            id: *fleet_id,
            fits: core_fleet.fits.clone(),
        };
        Ok(fleet)
    }
}
