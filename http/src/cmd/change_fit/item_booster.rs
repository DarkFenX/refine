use crate::cmd::{change_item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> rc::Result<rc::SolBoosterInfo> {
        core_sol.add_booster(*fit_id, self.type_id, self.state.unwrap_or(true))
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
