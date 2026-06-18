use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFleetId(usize);

impl ArenaId for UFleetId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
