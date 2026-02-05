use super::shared::{SeqAccum, SeqInstanceAccum};
use crate::{
    num::{Count, PValue},
    util::LibDefault,
};

impl<T> SeqAccum<BasicSeqInstanceAccum<T>> {
    pub(in crate::svc::vast) fn new_basic() -> Self
    where
        T: LibDefault,
    {
        SeqAccum {
            instances: BasicSeqInstanceAccum::default(),
            time: PValue::ZERO,
        }
    }
    pub(in crate::svc::vast) fn get_per_second(self) -> T
    where
        T: std::ops::Div<PValue, Output = T>,
    {
        self.instances.amount / self.time
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Instance accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct BasicSeqInstanceAccum<T> {
    pub(in crate::svc::vast) amount: T,
}
impl<T> Default for BasicSeqInstanceAccum<T>
where
    T: LibDefault,
{
    fn default() -> Self {
        BasicSeqInstanceAccum {
            amount: T::lib_default(),
        }
    }
}
impl<T> SeqInstanceAccum<T> for BasicSeqInstanceAccum<T>
where
    T: Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T> + std::ops::MulAssign<PValue>,
{
    fn add_instance(&mut self, mut instance: T, chance_mult: Option<PValue>, count: Count) {
        if let Some(chance_mult) = chance_mult {
            instance *= chance_mult;
        }
        self.amount += instance * count.into_pvalue();
    }
    fn merge(&mut self, other: &Self, count: Count) {
        self.amount += other.amount * count.into_pvalue();
    }
}
