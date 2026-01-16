use crate::num::PValue;

pub(in crate::svc) struct OutputIterItem<T> {
    pub(in crate::svc) time_passed: PValue,
    pub(in crate::svc) amount: T,
}
