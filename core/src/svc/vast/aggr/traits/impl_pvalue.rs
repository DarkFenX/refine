use super::{instance_duration::InstanceDuration, limit_amount::InstanceLimit};
use crate::{num::PValue, util::LibDefault};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceDuration for PValue {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
    fn limit_duration(&mut self, _limit: PValue) {}
}
impl InstanceLimit for PValue {
    fn instance_limit(&mut self, limit: PValue) {
        *self = PValue::min(*self, limit);
    }
}
impl LibDefault for PValue {
    fn lib_default() -> Self {
        PValue::ZERO
    }
}
