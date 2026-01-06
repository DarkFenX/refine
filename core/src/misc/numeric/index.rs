/// Float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct Index(usize);
impl Index {
    pub const fn from_usize(index: usize) -> Self {
        Self(index)
    }
    pub const fn into_usize(self) -> usize {
        self.0
    }
}
