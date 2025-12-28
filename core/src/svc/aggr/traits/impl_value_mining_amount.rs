use super::limit_amount::LimitAmount;
use crate::{def::AttrVal, misc::MiningAmount};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LimitAmount for MiningAmount {
    // No-op, since there is no logic to limit mining amount depending on target attrs
    fn limit_amount(&mut self, _limit: AttrVal) {}
}

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
impl std::ops::Mul<AttrVal> for MiningAmount {
    type Output = MiningAmount;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            drain: self.drain * rhs,
        }
    }
}
impl std::ops::MulAssign<AttrVal> for MiningAmount {
    fn mul_assign(&mut self, rhs: AttrVal) {
        self.yield_ *= rhs;
        self.drain *= rhs;
    }
}
impl std::ops::Div<AttrVal> for MiningAmount {
    type Output = MiningAmount;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            drain: self.drain / rhs,
        }
    }
}
