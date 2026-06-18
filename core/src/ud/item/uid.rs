use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, derive_more::Display)]
pub(crate) struct UItemId(usize);

impl ArenaId for UItemId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
