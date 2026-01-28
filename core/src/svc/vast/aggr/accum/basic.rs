use super::traits::StatAccumSynced;
use crate::{Count, PValue, util::LibDefault};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Synced
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct BasicAccumSynced<T> {
    total_amount: T,
}
impl<T> BasicAccumSynced<T>
where
    T: LibDefault,
{
    pub(in crate::svc::vast) fn new() -> Self {
        BasicAccumSynced {
            total_amount: T::lib_default(),
        }
    }
}
impl<T> StatAccumSynced<T> for BasicAccumSynced<T>
where
    T: std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T> + std::ops::MulAssign<PValue>,
{
    fn add_amount(&mut self, mut amount: T, chance_mult: Option<PValue>) {
        if let Some(chance_mult) = chance_mult {
            amount *= chance_mult;
        }
        self.total_amount += amount;
    }
    fn add_amount_multiple(&mut self, mut amount: T, chance_mult: Option<PValue>, count: Count) {
        if let Some(chance_mult) = chance_mult {
            amount *= chance_mult;
        }
        self.total_amount += amount * count.into_pvalue();
    }
}
