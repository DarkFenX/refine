#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFleetId(usize);
// Conversions needed for unified user entity container to work
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
