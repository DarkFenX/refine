use super::shared::{SeqAccum, SeqInstanceAccum};
use crate::{
    num::{Count, PValue},
    util::LibDefault,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Top-level accumulator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> SeqAccum<SeqInstanceAccumStack<T>> {
    pub(in crate::svc::vast) fn new_stack() -> Self
    where
        T: LibDefault,
    {
        SeqAccum {
            instances: SeqInstanceAccumStack::default(),
            time: PValue::ZERO,
        }
    }
    pub(in crate::svc::vast) fn get_per_second(self) -> T
    where
        T: std::ops::Div<PValue, Output = T>,
    {
        self.instances.stacked / self.time
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence accumulator implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct SeqInstanceAccumStack<T> {
    pub(in crate::svc::vast) stacked: T,
}
impl<T> Default for SeqInstanceAccumStack<T>
where
    T: LibDefault,
{
    fn default() -> Self {
        Self {
            stacked: T::lib_default(),
        }
    }
}
impl<T> SeqInstanceAccum<T> for SeqInstanceAccumStack<T>
where
    T: Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T> + std::ops::MulAssign<PValue>,
{
    fn add_instance(&mut self, mut instance: T, chance_mult: Option<PValue>, count: Count) {
        if let Some(chance_mult) = chance_mult {
            instance *= chance_mult;
        }
        self.stacked += instance * count.into_pvalue();
    }
    fn merge(&mut self, other: &Self, count: Count) {
        self.stacked += other.stacked * count.into_pvalue();
    }
}
