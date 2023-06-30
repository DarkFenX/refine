use crate::cmd::HCmdResp;

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    state: Option<bool>,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = self.state {
            core_ss.set_skill_state(item_id, state)?;
        }
        Ok(HCmdResp::NoData)
    }
}
