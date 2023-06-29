#[derive(serde::Deserialize)]
pub(crate) struct HChangeImplantCmd {
    state: Option<bool>,
}
impl HChangeImplantCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
