use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RBuffId(usize);

impl SlabId for RBuffId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
