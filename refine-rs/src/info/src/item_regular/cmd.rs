use crate::{ItemTypeId, src::SrcItemInfos};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SrcItemInfosCmd {
    /// None means all item types of the source.
    type_ids: Option<Vec<ItemTypeId>>,
}
const impl Default for SrcItemInfosCmd {
    fn default() -> Self {
        Self { type_ids: None }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfosCmd {
    /// Command which requests all item types, until it is limited by [`Self::with_type_ids`].
    pub const fn new() -> Self {
        Self::default()
    }
    pub fn with_type_ids(mut self, type_ids: impl IntoIterator<Item = ItemTypeId>) -> Self {
        self.type_ids.get_or_insert_default().extend(type_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfosCmd {
    pub(crate) fn execute(self, core_src: &rc::Src) -> SrcItemInfos {
        SrcItemInfos::from_core(core_src, self.type_ids.as_deref())
    }
}
