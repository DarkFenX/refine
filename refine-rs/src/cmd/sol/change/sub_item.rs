use crate::cmd::{ChangeSolEnumCmd, inner::ICmdItemRemoveFCtxBIds, shared::ItemIdBackref};

pub struct SolRemoveItemCmd {
    pub(super) inner: ICmdItemRemoveFCtxBIds,
}
impl SolRemoveItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdItemRemoveFCtxBIds { item_id, .. },
        }
    }
    pub fn with_rm_mode(mut self, rm_mode: rc::RemoveMode) -> Self {
        self.inner.ictx_cmd.rm_mode = Some(rm_mode);
        self
    }
}
impl From<SolRemoveItemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}
