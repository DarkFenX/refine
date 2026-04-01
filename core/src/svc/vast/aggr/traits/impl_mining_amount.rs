use super::{has_impact::HasImpact, instance_duration::InstanceDuration, limit_amount::InstanceLimit};
use crate::{nd::NEffectMiningAmount, num::PValue, util::LibDefault};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasImpact for NEffectMiningAmount {
    fn has_impact(&self) -> bool {
        self.yield_ != PValue::ZERO || self.drain != PValue::ZERO
    }
}
impl InstanceDuration for NEffectMiningAmount {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
    fn limit_duration(&mut self, _limit: PValue) {}
}
impl InstanceLimit for NEffectMiningAmount {
    // No-op, since there is no logic to limit mining amount depending on target attrs
    fn instance_limit(&mut self, _limit: PValue) {}
}
impl LibDefault for NEffectMiningAmount {
    fn lib_default() -> Self {
        Self {
            yield_: PValue::ZERO,
            drain: PValue::ZERO,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General traits
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PartialEq for NEffectMiningAmount {
    fn eq(&self, other: &Self) -> bool {
        self.yield_ == other.yield_ && self.drain == other.drain
    }
}
impl Eq for NEffectMiningAmount {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Add<NEffectMiningAmount> for NEffectMiningAmount {
    type Output = NEffectMiningAmount;
    fn add(self, rhs: NEffectMiningAmount) -> Self::Output {
        Self {
            yield_: self.yield_ + rhs.yield_,
            drain: self.drain + rhs.drain,
        }
    }
}
impl std::ops::AddAssign<NEffectMiningAmount> for NEffectMiningAmount {
    fn add_assign(&mut self, rhs: NEffectMiningAmount) {
        self.yield_ += rhs.yield_;
        self.drain += rhs.drain;
    }
}
impl std::ops::Mul<PValue> for NEffectMiningAmount {
    type Output = NEffectMiningAmount;
    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            drain: self.drain * rhs,
        }
    }
}
impl std::ops::MulAssign<PValue> for NEffectMiningAmount {
    fn mul_assign(&mut self, rhs: PValue) {
        self.yield_ *= rhs;
        self.drain *= rhs;
    }
}
impl std::ops::Div<PValue> for NEffectMiningAmount {
    type Output = NEffectMiningAmount;
    fn div(self, rhs: PValue) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            drain: self.drain / rhs,
        }
    }
}
