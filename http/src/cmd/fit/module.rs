use crate::{
    cmd::{item, shared::HAddMode},
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

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::ReeId,
    #[serde(flatten)]
    item_cmd: item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd::fit) fn from_item_cmd(item_id: rc::ReeId, item_cmd: item::HChangeModuleCmd) -> Self {
        Self { item_id, item_cmd }
    }
    pub(crate) fn get_item_id(&self) -> rc::ReeId {
        self.item_id
    }
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.item_cmd.get_state()
    }
}
