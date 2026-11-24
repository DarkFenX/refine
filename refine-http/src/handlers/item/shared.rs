use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ItemParams {
    #[serde(default)]
    item: rs::info::ItemInfoMode,
}
impl ItemParams {
    pub(super) fn into_cmd(self) -> rs::info::ItemInfoCmd {
        rs::info::ItemInfoCmd::new().with_item_default(self.item)
    }
    pub(super) fn into_cmd_br(self) -> rs::info::ItemInfoCmdBr {
        rs::info::ItemInfoCmdBr::new().with_item_default(self.item)
    }
}
