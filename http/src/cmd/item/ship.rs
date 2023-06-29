#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipCmd {
    state: Option<bool>,
}
impl HChangeShipCmd {
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.state
    }
}
