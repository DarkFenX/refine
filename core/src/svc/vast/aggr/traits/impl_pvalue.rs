use super::{instance_duration::InstanceDuration, limit_amount::LimitAmount};
use crate::num::{PValue, Value};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceDuration for PValue {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
    fn limit_duration(&mut self, _limit: PValue) {}
}
impl LimitAmount for PValue {
    fn limit_amount(&mut self, limit: Value) {
        *self = PValue::min(*self, PValue::from_value_clamped(limit));
    }
}
