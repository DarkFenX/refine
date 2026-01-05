#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UFleetId(usize);

impl From<usize> for UFleetId {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UFleetId> for usize {
    fn from(v: UFleetId) -> Self {
        v.0
    }
}
