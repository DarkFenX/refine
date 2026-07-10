use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFleetId(usize);

impl SlabId for UFleetId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
