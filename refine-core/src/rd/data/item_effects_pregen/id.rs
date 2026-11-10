use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct RItemEffectsPregenId(U32NonMax);
impl DenseId for RItemEffectsPregenId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}
// Placeholder for arrays filled after all items are built
impl Default for RItemEffectsPregenId {
    fn default() -> Self {
        Self(U32NonMax::ZERO)
    }
}
