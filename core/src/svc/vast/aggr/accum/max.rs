use super::shared::SeqInstanceAccum;
use crate::{
    num::{Count, PValue},
    util::{LibDefault, LibMax},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence accumulator implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct SeqInstanceAccumMax<I> {
    pub(in crate::svc::vast) max: I,
}
impl<I> SeqInstanceAccumMax<I>
where
    I: LibDefault,
{
    pub(in crate::svc::vast) fn new() -> Self {
        Self { max: I::lib_default() }
    }
}
impl<I> SeqInstanceAccum<I> for SeqInstanceAccumMax<I>
where
    I: Copy + std::ops::MulAssign<PValue> + LibDefault + LibMax,
{
    fn add_instance(&mut self, mut instance: I, chance_mult: Option<PValue>, _count: Count) {
        if let Some(chance_mult) = chance_mult {
            instance *= chance_mult;
        }
        self.max = LibMax::lib_max(self.max, instance);
    }
    fn copy_blank(&self) -> Self {
        Self::new()
    }
    fn merge(&mut self, other: &Self, _count: Count) {
        self.max = LibMax::lib_max(self.max, other.max);
    }
}
