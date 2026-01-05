#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFitId(usize);
impl From<usize> for UFitId {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UFitId> for usize {
    fn from(v: UFitId) -> Self {
        v.0
    }
}
