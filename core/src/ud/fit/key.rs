#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UFitKey(usize);

impl From<usize> for UFitKey {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UFitKey> for usize {
    fn from(v: UFitKey) -> Self {
        v.0
    }
}
