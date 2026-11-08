use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RItemListId(U32NonMax);

impl DenseId for RItemListId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}
