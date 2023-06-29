use crate::{cmd::item, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::EItemId,
    state: HState,
}
impl HAddDroneCmd {
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> &HState {
        &self.state
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeDroneCmd) -> Self {
        Self { item_id, item_cmd }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.item_id
    }
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.item_cmd.get_state()
    }
}
