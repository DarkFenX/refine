use crate::{
    cmd::{
        item,
        shared::{HAddMode, HEffectModeMap},
    },
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::EItemId,
    state: HState,
    charge_type_id: Option<rc::EItemId>,
}
impl HAddModuleCmd {
    pub(crate) fn get_rack(&self) -> &HModRack {
        &self.rack
    }
    pub(crate) fn get_add_mode(&self) -> &HAddMode {
        &self.add_mode
    }
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> &HState {
        &self.state
    }
    pub(crate) fn get_charge_type_id(&self) -> Option<rc::EItemId> {
        self.charge_type_id
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeModuleCmd) -> Self {
        Self { item_id, item_cmd }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.item_id
    }
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.item_cmd.get_state()
    }
    pub(crate) fn get_effect_modes(&self) -> Option<&HEffectModeMap> {
        self.item_cmd.get_effect_modes()
    }
}
