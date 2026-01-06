#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(usize);
impl RAttrId {
    pub(in crate::rd) fn from_usize(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_usize(self) -> usize {
        self.0
    }
}
