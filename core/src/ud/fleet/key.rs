#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UFleetKey(usize);

impl From<usize> for UFleetKey {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UFleetKey> for usize {
    fn from(v: UFleetKey) -> Self {
        v.0
    }
}
