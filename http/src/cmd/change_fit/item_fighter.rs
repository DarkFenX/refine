use crate::{
    cmd::{HCmdResp, change_item},
    shared::HMinionState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: rc::ItemTypeId,
    state: HMinionState,
    count: Option<rc::Count>,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::FighterInfo, HExecError> {
        let core_fighter = match core_sol.add_fighter(*fit_id, self.type_id, (&self.state).into()) {
            Ok(core_fighter) => core_fighter,
            Err(error) => {
                return Err(match error {
                    rc::err::AddFighterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        if let Some(count) = self.count {
            if let Err(error) = core_sol.set_fighter_count_override(&core_fighter.id, count) {
                return Err(match error {
                    rc::err::SetFighterCountOverrideError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetFighterCountOverrideError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
                    rc::err::SetFighterCountOverrideError::FighterCountError(e) => HExecError::InvalidFighterCount(e),
                });
            }
        }
        let core_fighter = core_sol.get_fighter(&core_fighter.id).unwrap();
        Ok(core_fighter)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
