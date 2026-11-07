use crate::util::DenseId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RBuffId(u32);

impl DenseId for RBuffId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}
