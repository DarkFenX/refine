#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceCmd {
    state: Option<bool>,
}
impl HChangeStanceCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
