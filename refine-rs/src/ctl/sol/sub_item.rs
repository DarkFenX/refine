use crate::{ItemIdBackref, RemoveMode, SolCtlCmd, ctl::core::ICmdItemRemoveFCtxBIds};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolRemoveItemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdItemRemoveFCtxBIds,
}
impl SolRemoveItemCmd {
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
impl From<SolRemoveItemCmd> for SolCtlCmd {
    fn from(sub_cmd: SolRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}
