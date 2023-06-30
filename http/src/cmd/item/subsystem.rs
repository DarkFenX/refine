#[derive(serde::Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    state: Option<bool>,
}
impl HChangeSubsystemCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
