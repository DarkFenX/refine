use super::accum::StatAccum;
use crate::{Count, PValue, util::LibDefault};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic accumulator which tracks just output amount
////////////////////////////////////////////////////////////////////////////////////////////////////
struct BasicAccum<T> {
    total_amount: T,
}
impl<T> BasicAccum<T>
where
    T: LibDefault,
{
    pub(in crate::svc::vast::aggr) fn new() -> Self {
        BasicAccum {
            total_amount: T::lib_default(),
        }
    }
}
impl<T> StatAccum<T, T> for BasicAccum<T>
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
    fn get_stat(self) -> T {
        self.total_amount
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular accumulator where
////////////////////////////////////////////////////////////////////////////////////////////////////
