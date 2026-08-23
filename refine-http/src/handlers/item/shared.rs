use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ItemParams {
    #[serde(default)]
    item: rs::ItemInfoMode,
}
impl ItemParams {
    pub(super) fn into_cmd(self) -> rs::ItemInfoCmd {
        rs::ItemInfoCmd::new().with_item_default(self.item)
    }
    pub(super) fn into_cmd_br(self) -> rs::ItemInfoCmdBr {
        rs::ItemInfoCmdBr::new().with_item_default(self.item)
    }
}
