use crate::{
    cmd::{change_item, HCmdResp},
    shared::HState,
    util::HExecError,
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
    ) -> Result<rc::SolFighterInfo, HExecError> {
        let core_fighter = match core_sol.add_fighter(*fit_id, self.type_id, (&self.state).into()) {
            Ok(core_fighter) => core_fighter,
            Err(error) => {
                return Err(match error {
                    rc::err::AddFighterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                })
            }
        };
        Ok(core_fighter)
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
