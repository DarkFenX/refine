use crate::{
    cmd::shared::HAddMode,
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) rack: HModRack,
    pub(crate) add_mode: HAddMode,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
}
impl HAddModuleCmd {
    pub(crate) fn new(
        fit_id: rc::ReeId,
        rack: HModRack,
        add_mode: HAddMode,
        type_id: rc::ReeInt,
        state: HState,
        charge_type_id: Option<rc::ReeInt>,
    ) -> Self {
        Self {
            fit_id,
            rack,
            add_mode,
            type_id,
            charge_type_id,
            state,
        }
    }
}
