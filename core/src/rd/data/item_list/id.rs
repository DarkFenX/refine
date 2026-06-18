use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RItemListId(usize);

impl ArenaId for RItemListId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
