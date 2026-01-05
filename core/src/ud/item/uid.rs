#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UItemId(usize);
impl From<usize> for UItemId {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UItemId> for usize {
    fn from(v: UItemId) -> Self {
        v.0
    }
}
