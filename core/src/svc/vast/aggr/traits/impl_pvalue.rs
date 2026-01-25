use super::{get_duration::GetDuration, limit_amount::LimitAmount};
use crate::num::{PValue, Value};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetDuration for PValue {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
}
impl LimitAmount for PValue {
    fn limit_amount(&mut self, limit: Value) {
        *self = PValue::min(*self, PValue::from_value_clamped(limit));
    }
}
