use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RBuffId(usize);

impl ArenaId for RBuffId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
