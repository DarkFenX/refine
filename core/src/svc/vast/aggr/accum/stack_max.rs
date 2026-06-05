use super::shared::{SeqAccum, SeqInstanceAccum};
use crate::{
    num::{Count, PValue},
    util::{LibDefault, LibMax},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Top-level accumulator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> SeqAccum<SeqInstanceAccumStackMax<I>> {
    pub(in crate::svc::vast) fn new_stack_max() -> Self
    where
        I: LibDefault,
    {
        SeqAccum {
            instances: SeqInstanceAccumStackMax::new(),
            time: PValue::ZERO,
        }
    }
    pub(in crate::svc::vast) fn get_per_second(&self) -> I
    where
        I: Copy + std::ops::Div<PValue, Output = I>,
    {
        self.instances.stacked / self.time
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence accumulator implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct SeqInstanceAccumStackMax<I> {
    pub(in crate::svc::vast) stacked: I,
    pub(in crate::svc::vast) max: I,
}
impl<I> SeqInstanceAccumStackMax<I>
where
    I: LibDefault,
{
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            stacked: I::lib_default(),
            max: I::lib_default(),
        }
    }
}
impl<I> SeqInstanceAccum<I> for SeqInstanceAccumStackMax<I>
where
    I: Copy
        + std::ops::AddAssign<I>
        + std::ops::Mul<PValue, Output = I>
        + std::ops::MulAssign<PValue>
        + LibDefault
        + LibMax,
{
    fn add_instance(&mut self, mut instance: I, chance_mult: Option<PValue>, count: Count) {
        if let Some(chance_mult) = chance_mult {
            instance *= chance_mult;
        }
        self.stacked += instance * count.into_pvalue();
        self.max = LibMax::lib_max(self.max, instance);
    }
    fn copy_blank(&self) -> Self {
        Self::new()
    }
    fn merge(&mut self, other: &Self, count: Count) {
        self.stacked += other.stacked * count.into_pvalue();
        self.max = LibMax::lib_max(self.max, other.max);
    }
}
