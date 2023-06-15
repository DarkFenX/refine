use crate::cmd::ss;

#[derive(serde::Deserialize)]
pub(crate) struct HSetCharCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetCharCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HSetCharCmd {
        ss::HSetCharCmd::new(fit_id, self.type_id, self.state)
    }
}
