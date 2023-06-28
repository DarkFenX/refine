#[derive(serde::Deserialize)]
pub(crate) struct HSetCharCmd {
    type_id: rc::ItemId,
    state: Option<bool>,
}
impl HSetCharCmd {
    pub(crate) fn get_type_id(&self) -> rc::ItemId {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> bool {
        self.state.unwrap_or(true)
    }
}
