use crate::{
    RemoveMode,
    cmd::inner::{ICmdItemRemoveICtx, ItemRemoveItemError},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct RemoveItemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: ICmdItemRemoveICtx = ICmdItemRemoveICtx { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RemoveItemCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_rm_mode(mut self, rm_mode: RemoveMode) -> Self {
        self.inner.rm_mode = Some(rm_mode);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RemoveItemCmd {
    pub(crate) fn execute(self, core_item: rc::ItemMut) -> Result<(), ItemRemoveItemError> {
        self.inner.execute(core_item)
    }
}
