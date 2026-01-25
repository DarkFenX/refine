use super::{get_duration::GetDuration, limit_amount::LimitAmount};
use crate::{
    misc::MiningAmount,
    num::{PValue, Value},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetDuration for MiningAmount {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
}
impl LimitAmount for MiningAmount {
    // No-op, since there is no logic to limit mining amount depending on target attrs
    fn limit_amount(&mut self, _limit: Value) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General traits
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PartialEq for MiningAmount {
    fn eq(&self, other: &Self) -> bool {
        self.yield_ == other.yield_ && self.drain == other.drain
    }
}
impl Eq for MiningAmount {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Add<MiningAmount> for MiningAmount {
    type Output = MiningAmount;
    fn add(self, rhs: MiningAmount) -> Self::Output {
        Self {
            yield_: self.yield_ + rhs.yield_,
            drain: self.drain + rhs.drain,
        }
    }
}
impl std::ops::AddAssign<MiningAmount> for MiningAmount {
    fn add_assign(&mut self, rhs: MiningAmount) {
        self.yield_ += rhs.yield_;
        self.drain += rhs.drain;
    }
}
impl std::ops::Mul<PValue> for MiningAmount {
    type Output = MiningAmount;
    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            drain: self.drain * rhs,
        }
    }
}
impl std::ops::MulAssign<PValue> for MiningAmount {
    fn mul_assign(&mut self, rhs: PValue) {
        self.yield_ *= rhs;
        self.drain *= rhs;
    }
}
impl std::ops::Div<PValue> for MiningAmount {
    type Output = MiningAmount;
    fn div(self, rhs: PValue) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            drain: self.drain / rhs,
        }
    }
}
