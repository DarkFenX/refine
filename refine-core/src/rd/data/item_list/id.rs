use std::num::NonZeroU32;

use crate::util::DenseId;

/// Imitates non-max u32, since pattern types are not really usable even in nightly.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RItemListId(NonZeroU32);

impl DenseId for RItemListId {
    fn new(index: usize) -> Self {
        Self(NonZeroU32::MIN.checked_add(index.try_into().unwrap()).unwrap())
    }
    fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}
