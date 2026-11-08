use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct RMutasPregenMergedAttrsId(U32NonMax);
impl DenseId for RMutasPregenMergedAttrsId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct RMutasPregenFlexDataId(U32NonMax);
impl DenseId for RMutasPregenFlexDataId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}
