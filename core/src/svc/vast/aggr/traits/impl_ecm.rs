use super::{get_duration::GetDuration, limit_amount::LimitAmount};
use crate::{
    misc::Ecm,
    num::{PValue, Value},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetDuration for Ecm {
    fn get_duration(&self) -> PValue {
        self.duration
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
