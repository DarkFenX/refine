use super::{instance_duration::InstanceDuration, limit_amount::LimitInstance};
use crate::{
    num::{PValue, Value},
    util::LibDefault,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceDuration for PValue {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
    fn limit_duration(&mut self, _limit: PValue) {}
}
impl LimitInstance for PValue {
    fn limit_instance(&mut self, limit: Value) {
        *self = PValue::min(*self, PValue::from_value_clamped(limit));
    }
}
impl LibDefault for PValue {
    fn lib_default() -> Self {
        PValue::ZERO
    }
}
