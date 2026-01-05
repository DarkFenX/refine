#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RBuffId(usize);
impl RBuffId {
    pub(in crate::rd) fn new(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_inner(self) -> usize {
        self.0
    }
}
