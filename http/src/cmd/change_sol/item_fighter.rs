use crate::{
    cmd::{change_fit, HCmdResp},
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddFighterCmd,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<rc::SolFighterInfo> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.fit_cmd.execute(core_sol)
    }
}
