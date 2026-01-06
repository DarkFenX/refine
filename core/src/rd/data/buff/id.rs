#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RBuffId(usize);
impl RBuffId {
    pub(in crate::rd) fn from_usize(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_usize(self) -> usize {
        self.0
    }
}
