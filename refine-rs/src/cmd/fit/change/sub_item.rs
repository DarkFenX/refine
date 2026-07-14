use crate::cmd::{ChangeFitEnumCmd, inner::ICmdItemRemoveFCtxBIds, shared::ItemIdBackref};

pub struct FitRemoveItemCmd {
    pub(super) inner: ICmdItemRemoveFCtxBIds,
}
impl FitRemoveItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdItemRemoveFCtxBIds { item_id, .. },
        }
    }
    pub fn with_rm_mode(mut self, rm_mode: rc::RmMode) -> Self {
        self.inner.ictx_cmd.rm_mode = Some(rm_mode);
        self
    }
}
impl From<FitRemoveItemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}
