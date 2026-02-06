use super::shared::{SeqAccum, SeqInstanceAccum};
use crate::{
    num::{Count, PValue},
    util::{LibDefault, LibMax},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Top-level accumulator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> SeqAccum<SeqInstanceAccumStackMax<T>> {
    pub(in crate::svc::vast) fn new_stack_max() -> Self
    where
        T: LibDefault,
    {
        SeqAccum {
            instances: SeqInstanceAccumStackMax::new(),
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
pub(in crate::svc::vast) struct SeqInstanceAccumStackMax<T> {
    pub(in crate::svc::vast) stacked: T,
    pub(in crate::svc::vast) max: T,
}
impl<T> SeqInstanceAccumStackMax<T>
where
    T: LibDefault,
{
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            stacked: T::lib_default(),
            max: T::lib_default(),
        }
    }
}
impl<T> SeqInstanceAccum<T> for SeqInstanceAccumStackMax<T>
where
    T: Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LibDefault
        + LibMax,
{
    fn add_instance(&mut self, mut instance: T, chance_mult: Option<PValue>, count: Count) {
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
