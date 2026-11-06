use std::num::Wrapping;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct Generation(Wrapping<u32>);
impl Generation {
    /// Increment counter, returning true if it wrapped.
    pub(super) fn increment(&mut self) -> bool {
        self.0 += Wrapping(1);
        *self == Self::default()
    }
}
const impl Default for Generation {
    fn default() -> Self {
        Self(Wrapping(0))
    }
}
