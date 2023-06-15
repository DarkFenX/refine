use crate::cmd::ss;

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetShipCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HSetShipCmd {
        ss::HSetShipCmd::new(fit_id, self.type_id, self.state)
    }
}
