use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(usize);

impl ArenaId for RAttrId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
