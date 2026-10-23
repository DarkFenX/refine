use super::{has_impact::HasImpact, instance_duration::InstanceDuration, limit_amount::InstanceLimit};
use crate::{nd::NEffectEcmAmount, num::PValue};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasImpact for NEffectEcmAmount {
    fn has_impact(&self) -> bool {
        let Self {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            // Jam duration without jam strength has no impact
            duration: _,
        } = self;
        *radar != PValue::ZERO
            || *magnetometric != PValue::ZERO
            || *gravimetric != PValue::ZERO
            || *ladar != PValue::ZERO
    }
}
impl InstanceDuration for NEffectEcmAmount {
    fn get_duration(&self) -> PValue {
        self.duration
    }
    fn limit_duration(&mut self, limit: PValue) {
        self.duration = self.duration.min(limit);
    }
}
impl InstanceLimit for NEffectEcmAmount {
    // No-op, since there is no logic to limit ECM depending on target attrs
    fn instance_limit(&mut self, _limit: PValue) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Mul<PValue> for NEffectEcmAmount {
    type Output = Self;
    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            radar: self.radar * rhs,
            magnetometric: self.magnetometric * rhs,
            gravimetric: self.gravimetric * rhs,
            ladar: self.ladar * rhs,
            duration: self.duration,
        }
    }
}
impl std::ops::MulAssign<PValue> for NEffectEcmAmount {
    fn mul_assign(&mut self, rhs: PValue) {
        let Self {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: _,
        } = self;
        *radar *= rhs;
        *magnetometric *= rhs;
        *gravimetric *= rhs;
        *ladar *= rhs;
    }
}
