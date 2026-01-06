/// Float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct Index(usize);
impl Index {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }
    pub(crate) fn into_inner(self) -> usize {
        self.0
    }
}
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
impl From<Index> for usize {
    fn from(value: Index) -> Self {
        value.0
    }
}
