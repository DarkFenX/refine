use crate::{Drone, RangedProjIter};

impl<'s> Drone<'s> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.uid)
    }
}
