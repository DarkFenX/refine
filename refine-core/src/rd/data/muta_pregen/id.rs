use crate::util::DenseId;

// IDs are u32 to keep key-to-ID maps and mutation caches smaller
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct RMutasPregenMergedAttrsId(u32);
impl DenseId for RMutasPregenMergedAttrsId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct RMutasPregenFlexDataId(u32);
impl DenseId for RMutasPregenFlexDataId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}
