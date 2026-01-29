use super::{instance_duration::InstanceDuration, limit_amount::LimitAmount};
use crate::{
    misc::Ecm,
    num::{PValue, Value},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceDuration for Ecm {
    fn get_duration(&self) -> PValue {
        self.duration
    }
    fn limit_duration(&mut self, limit: PValue) {
        self.duration = self.duration.min(limit);
    }
}
impl LimitAmount for Ecm {
    // No-op, since there is no logic to limit ECM depending on target attrs
    fn limit_amount(&mut self, _limit: Value) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::MulAssign<PValue> for Ecm {
    fn mul_assign(&mut self, rhs: PValue) {
        self.radar *= rhs;
        self.magnetometric *= rhs;
        self.gravimetric *= rhs;
        self.ladar *= rhs;
    }
}
