use derive_more::Display;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display)]
pub(crate) struct UItemKey(usize);

impl From<usize> for UItemKey {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<UItemKey> for usize {
    fn from(v: UItemKey) -> Self {
        v.0
    }
}
