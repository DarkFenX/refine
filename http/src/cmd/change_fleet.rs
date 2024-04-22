#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_fits: Vec<rc::SolFitId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    remove_fits: Vec<rc::SolFitId>,
}
impl HChangeFleetCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::SolItemId,
    ) -> rc::Result<rc::SolFleetInfo> {
        for fit_id in self.remove_fits.iter() {
            let _ = core_sol.set_fit_fleet(fit_id, None);
        }
        for fit_id in self.add_fits.iter() {
            let _ = core_sol.set_fit_fleet(fit_id, Some(*fleet_id));
        }
        core_sol.get_fleet(fleet_id)
    }
}
