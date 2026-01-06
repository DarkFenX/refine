#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UItemId(usize);
// Conversions needed for unified user entity container to work
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
