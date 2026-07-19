use crate::{ChangeFitEnumCmd, ItemIdBackref, RemoveMode, cmd::inner::ICmdItemRemoveFCtxBIds};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitRemoveItemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdItemRemoveFCtxBIds,
}
impl FitRemoveItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdItemRemoveFCtxBIds { item_id, .. },
        }
    }
    pub fn with_rm_mode(mut self, rm_mode: RemoveMode) -> Self {
        self.inner.ictx_cmd.rm_mode = Some(rm_mode);
        self
    }
}
impl From<FitRemoveItemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}
