use crate::{cmd::ss, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
}
impl HAddDroneCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddDroneCmd {
        ss::HAddDroneCmd::new(fit_id, self.type_id, self.state)
    }
}
