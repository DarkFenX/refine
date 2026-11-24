use crate::{ItemTypeId, src::SrcMutatedItemInfos};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SrcMutatedItemInfosCmd {
    mutations: Vec<SrcMutation>,
}
const impl Default for SrcMutatedItemInfosCmd {
    fn default() -> Self {
        Self {
            mutations: Default::default(),
        }
    }
}

/// Base item type and mutator which is applied to it.
#[cfg_attr(feature = "serde", derive(serde_tuple::Deserialize_tuple))]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SrcMutation {
    pub base_type_id: ItemTypeId,
    pub mutator_id: ItemTypeId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedItemInfosCmd {
    pub const fn new() -> Self {
        Self::default()
    }
    pub fn with_mutations(mut self, mutations: impl IntoIterator<Item = SrcMutation>) -> Self {
        self.mutations.extend(mutations);
        self
    }
}

impl SrcMutation {
    pub const fn new(base_type_id: ItemTypeId, mutator_id: ItemTypeId) -> Self {
        Self {
            base_type_id,
            mutator_id,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedItemInfosCmd {
    pub(crate) fn execute(self, core_src: &rc::Src) -> SrcMutatedItemInfos {
        SrcMutatedItemInfos::from_core(core_src, &self.mutations)
    }
}
