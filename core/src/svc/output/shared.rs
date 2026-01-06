use crate::misc::PValue;

pub(in crate::svc) struct OutputIterItem<T> {
    pub(in crate::svc) time: PValue,
    pub(in crate::svc) amount: T,
}
