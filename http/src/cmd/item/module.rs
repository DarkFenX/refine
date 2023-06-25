use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HState>,
}
impl HChangeModuleCmd {
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.state.as_ref()
    }
}
