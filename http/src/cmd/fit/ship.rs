#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: rc::ReeInt,
    state: Option<bool>,
}
impl HSetShipCmd {
    pub(crate) fn get_type_id(&self) -> rc::ReeInt {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> bool {
        self.state.unwrap_or(true)
    }
}
