use crate::util::DenseId;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct RItemEffectsPregenId(u32);
impl DenseId for RItemEffectsPregenId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}
