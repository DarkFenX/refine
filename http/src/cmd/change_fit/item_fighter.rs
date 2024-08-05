use crate::{
    cmd::{change_item, HCmdResp},
    shared::HState,
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: rc::EItemId,
    state: HState,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<rc::SolFighterInfo> {
        let info = core_sol.add_fighter(*fit_id, self.type_id, (&self.state).into())?;
        Ok(info)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
