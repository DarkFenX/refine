use crate::util::ArenaId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct REffectId(usize);

impl ArenaId for REffectId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
    }
}
