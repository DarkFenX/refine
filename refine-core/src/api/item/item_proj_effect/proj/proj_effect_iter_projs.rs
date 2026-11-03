use crate::{ProjEffect, ProjIter};

impl<'s> ProjEffect<'s> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&mut self) -> ProjIter<'_> {
        ProjIter::new(self.sol, self.uid)
    }
}
