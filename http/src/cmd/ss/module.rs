use crate::cmd::shared::{HAddMode, HState};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) add_mode: HAddMode,
    pub(crate) module_type_id: rc::ReeInt,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
    pub(crate) state: HState,
}
impl HAddModuleCmd {
    pub(crate) fn new(
        fit_id: rc::ReeId,
        add_mode: HAddMode,
        module_type_id: rc::ReeInt,
        charge_type_id: Option<rc::ReeInt>,
        state: HState,
    ) -> Self {
        Self {
            fit_id,
            add_mode,
            module_type_id,
            charge_type_id,
            state,
        }
    }
}
