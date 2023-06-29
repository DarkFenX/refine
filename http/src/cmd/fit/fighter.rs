use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: rc::EItemId,
    state: HState,
}
impl HAddFighterCmd {
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> &HState {
        &self.state
    }
}
