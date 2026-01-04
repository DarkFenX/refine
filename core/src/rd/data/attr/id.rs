#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(usize);
impl RAttrId {
    pub(in crate::rd) fn new(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_inner(self) -> usize {
        self.0
    }
}
