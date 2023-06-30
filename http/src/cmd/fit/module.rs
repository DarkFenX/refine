use crate::{
    cmd::{item, shared::HAddMode, HCmdResp},
    shared::{HModRack, HState},
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
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .add_module(
                *fit_id,
                (&self.rack).into(),
                (&self.add_mode).into(),
                self.type_id,
                (&self.state).into(),
                self.charge_type_id,
            )?
            .into())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
