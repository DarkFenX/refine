#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_fits: Vec<rc::SsFitId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    remove_fits: Vec<rc::SsFitId>,
}
impl HChangeFleetCmd {
    pub(crate) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        fleet_id: &rc::SsItemId,
    ) -> rc::Result<rc::SsFleetInfo> {
        for fit_id in self.remove_fits.iter() {
            let _ = core_ss.set_fit_fleet(fit_id, None);
        }
        for fit_id in self.add_fits.iter() {
            let _ = core_ss.set_fit_fleet(fit_id, Some(*fleet_id));
        }
        core_ss.get_fleet(fleet_id)
    }
}
