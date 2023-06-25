use crate::{
    cmd::shared::HAddMode,
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::ReeInt,
    state: HState,
    charge_type_id: Option<rc::ReeInt>,
}
impl HAddModuleCmd {
    pub(crate) fn get_rack(&self) -> &HModRack {
        &self.rack
    }
    pub(crate) fn get_add_mode(&self) -> &HAddMode {
        &self.add_mode
    }
    pub(crate) fn get_type_id(&self) -> rc::ReeInt {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> &HState {
        &self.state
    }
    pub(crate) fn get_charge_type_id(&self) -> Option<rc::ReeInt> {
        self.charge_type_id
    }
}
