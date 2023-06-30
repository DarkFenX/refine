use crate::{cmd::HCmdResp, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    state: Option<HState>,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = &self.state {
            core_ss.set_fighter_state(item_id, state.into())?;
        }
        Ok(HCmdResp::NoData)
    }
}
