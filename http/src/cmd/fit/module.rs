use crate::{
    cmd::{shared::HAddMode, ss},
    shared::HState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    pub(crate) add_mode: HAddMode,
    pub(crate) module_type_id: rc::ReeInt,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
    pub(crate) state: HState,
}
impl HAddModuleCmd {
    pub(in crate::cmd::fit) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddModuleCmd {
        ss::HAddModuleCmd::new(
            fit_id,
            self.add_mode,
            self.module_type_id,
            self.charge_type_id,
            self.state,
        )
    }
}
