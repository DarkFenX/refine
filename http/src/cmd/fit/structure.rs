use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HSetStructureCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HSetStructureCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .set_fit_structure(*fit_id, self.type_id, self.state.unwrap_or(true))?
            .into())
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStructureCmd {
    ViaItemId(HChangeStructureViaItemIdCmd),
    ViaFitId(HChangeStructureViaFitIdCmd),
}
impl HChangeStructureCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_ss),
            Self::ViaFitId(cmd) => cmd.execute(core_ss, fit_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeStructureViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeStructureCmd,
}
impl HChangeStructureViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStructureViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: item::HChangeStructureCmd,
}
impl HChangeStructureViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        let item_id = core_ss.get_fit_structure_info(fit_id)?.id;
        self.item_cmd.execute(core_ss, &item_id)
    }
}
