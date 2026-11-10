use crate::{num::U32NonMax, util::DenseId};

/// Item types with equal effect lists share the ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RItemEffectListId(U32NonMax);
impl DenseId for RItemEffectListId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}
// Placeholder for the field filled after all items are built
impl Default for RItemEffectListId {
    fn default() -> Self {
        Self(U32NonMax::ZERO)
    }
}
