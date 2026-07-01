use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct REffectId(usize);

impl SlabId for REffectId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
