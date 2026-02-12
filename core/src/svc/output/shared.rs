use crate::num::PValue;

pub(in crate::svc) struct OutputInstanceIterItem<T> {
    pub(in crate::svc) time_passed: PValue,
    pub(in crate::svc) instance: T,
}
