use crate::cmd::HCmdResp;

#[derive(serde::Deserialize)]
pub(crate) struct HChangeRigCmd {
    state: Option<bool>,
}
impl HChangeRigCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = self.state {
            core_ss.set_rig_state(item_id, state)?;
        }
        Ok(HCmdResp::NoData)
    }
}
