use crate::{
    cmd::{shared::HAddMode, ss},
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    pub(crate) rack: HModRack,
    pub(crate) add_mode: HAddMode,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
}
impl HAddModuleCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddModuleCmd {
        ss::HAddModuleCmd::new(
            fit_id,
            self.rack,
            self.add_mode,
            self.type_id,
            self.state,
            self.charge_type_id,
        )
    }
}
