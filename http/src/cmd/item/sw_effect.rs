#[derive(serde::Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    state: Option<bool>,
}
impl HChangeSwEffectCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
