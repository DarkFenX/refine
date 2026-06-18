use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFitId(usize);

impl ArenaId for UFitId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
