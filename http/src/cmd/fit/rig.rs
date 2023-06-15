use crate::cmd::ss;

#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddRigCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddRigCmd {
        ss::HAddRigCmd::new(fit_id, self.type_id, self.state)
    }
}
