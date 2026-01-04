#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct REffectId(usize);
impl REffectId {
    pub(in crate::rd) fn new(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_inner(self) -> usize {
        self.0
    }
}
