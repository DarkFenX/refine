#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    state: Option<bool>,
}
impl HChangeSkillCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
