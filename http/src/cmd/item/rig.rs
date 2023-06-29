#[derive(serde::Deserialize)]
pub(crate) struct HChangeRigCmd {
    state: Option<bool>,
}
impl HChangeRigCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
