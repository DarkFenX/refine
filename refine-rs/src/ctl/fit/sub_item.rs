use crate::{FitCtlCmd, ItemIdBackref, RemoveMode, ctl::core::ICmdItemRemoveFCtxBIds};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitRemoveItemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdItemRemoveFCtxBIds,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveItemCmd {
    pub fn into_ctl(self) -> FitCtlCmd {
        FitCtlCmd::RemoveItem(self)
    }
}
