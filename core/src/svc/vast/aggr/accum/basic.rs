use super::traits::SeqAccum;
use crate::{
    num::{Count, PValue},
    util::LibDefault,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Synced
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct BasicSeqAccum<T> {
    pub(in crate::svc::vast) amount: T,
}
impl<T> Default for BasicSeqAccum<T>
where
    T: LibDefault,
{
    fn default() -> Self {
        BasicSeqAccum {
            amount: T::lib_default(),
        }
    }
}
impl<T> SeqAccum<T> for BasicSeqAccum<T>
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
