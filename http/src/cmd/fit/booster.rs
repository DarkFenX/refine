use crate::cmd::item;

#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddBoosterCmd {
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.type_id
    }
    pub(crate) fn get_state(&self) -> bool {
        self.state.unwrap_or(true)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd::fit) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeBoosterCmd) -> Self {
        Self { item_id, item_cmd }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.item_id
    }
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.item_cmd.get_state()
    }
}
