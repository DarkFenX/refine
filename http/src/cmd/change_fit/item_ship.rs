use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<rc::SolShipInfo> {
        let info = core_sol.set_fit_ship(*fit_id, self.type_id, self.state.unwrap_or(true))?;
        Ok(info)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<HCmdResp> {
        let item_id = core_sol.get_fit_ship(fit_id)?.id;
        self.item_cmd.execute(core_sol, &item_id)
    }
}
