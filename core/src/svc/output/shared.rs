use crate::def::AttrVal;

pub(in crate::svc) struct OutputIterItem<T> {
    pub(in crate::svc) time: AttrVal,
    pub(in crate::svc) amount: T,
}
