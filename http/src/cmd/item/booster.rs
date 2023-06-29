#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    state: Option<bool>,
}
impl HChangeBoosterCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
