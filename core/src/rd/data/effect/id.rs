#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct REffectId(usize);
impl REffectId {
    pub(in crate::rd) fn from_usize(id: usize) -> Self {
        Self(id)
    }
    pub(in crate::rd) fn into_usize(self) -> usize {
        self.0
    }
}
