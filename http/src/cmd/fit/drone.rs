use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::ReeInt,
    state: HState,
}
impl HAddDroneCmd {
    pub(crate) fn get_type_id(&self) -> rc::ReeInt {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> &HState {
        &self.state
    }
}
