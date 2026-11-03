use crate::{Module, RangedProjIter};

impl<'s> Module<'s> {
    /// Iterates over module's projections.
    pub fn iter_projs(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.uid)
    }
}
