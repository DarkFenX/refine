#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterCmd {
    state: Option<bool>,
}
impl HChangeCharacterCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
