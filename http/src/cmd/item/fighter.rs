use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    state: Option<HState>,
}
impl HChangeFighterCmd {
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.state.as_ref()
    }
}
