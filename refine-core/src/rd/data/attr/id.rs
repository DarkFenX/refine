use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(usize);

impl SlabId for RAttrId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
