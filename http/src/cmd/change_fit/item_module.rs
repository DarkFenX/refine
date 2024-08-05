use crate::{
    cmd::{change_item, shared::HAddMode, HCmdResp},
    shared::{HModRack, HState},
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::EItemId,
    state: HState,
    charge_type_id: Option<rc::EItemId>,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<rc::SolModuleInfo> {
        let info = core_sol.add_module(
            *fit_id,
            (&self.rack).into(),
            (&self.add_mode).into(),
            self.type_id,
            (&self.state).into(),
            self.charge_type_id,
        )?;
        Ok(info)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
