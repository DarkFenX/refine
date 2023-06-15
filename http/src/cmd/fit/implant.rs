use crate::cmd::ss;

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddImplantCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddImplantCmd {
        ss::HAddImplantCmd::new(fit_id, self.type_id, self.state)
    }
}
