use crate::{cmd::HCmdResp, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    state: Option<HState>,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = &self.state {
            core_ss.set_drone_state(item_id, state.into())?;
        }
        Ok(HCmdResp::NoData)
    }
}
