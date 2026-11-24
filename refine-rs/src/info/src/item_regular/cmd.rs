use crate::{ItemTypeId, src::SrcItemInfos};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SrcItemInfosCmd {
    type_ids: Vec<ItemTypeId>,
}
const impl Default for SrcItemInfosCmd {
    fn default() -> Self {
        Self {
            type_ids: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfosCmd {
    pub const fn new() -> Self {
        Self::default()
    }
    pub fn with_type_ids(mut self, type_ids: impl IntoIterator<Item = ItemTypeId>) -> Self {
        self.type_ids.extend(type_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfosCmd {
    pub(crate) fn execute(self, core_src: &rc::Src) -> SrcItemInfos {
        SrcItemInfos::from_core(core_src, &self.type_ids)
    }
}
