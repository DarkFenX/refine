use crate::util::DenseId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFleetId(usize);

impl DenseId for UFleetId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
