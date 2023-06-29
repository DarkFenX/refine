use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    state: Option<HState>,
}
impl HChangeDroneCmd {
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.state.as_ref()
    }
}
