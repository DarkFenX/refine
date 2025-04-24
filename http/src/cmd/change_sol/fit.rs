use crate::{
    cmd::{HFitIdResp, change_fit, shared::get_primary_fit},
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeFitCmd,
}
impl HChangeFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFitIdResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
}
impl HDeleteFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        get_primary_fit(core_sol, &self.fit_id)?.remove();
        Ok(())
    }
}
