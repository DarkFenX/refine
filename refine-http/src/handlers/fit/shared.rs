use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FitParams {
    #[serde(default)]
    fit: rs::FitInfoMode,
    #[serde(default)]
    item: rs::ItemInfoMode,
}
impl FitParams {
    pub(super) fn into_cmd(self) -> rs::FitInfoCmd {
        rs::FitInfoCmd::new().with_fit(self.fit).with_item_default(self.item)
    }
    pub(super) fn into_cmd_br(self) -> rs::FitInfoCmdBr {
        rs::FitInfoCmdBr::new().with_fit(self.fit).with_item_default(self.item)
    }
}
