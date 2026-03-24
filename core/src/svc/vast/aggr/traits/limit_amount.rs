use crate::{
    num::PValue,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(crate) trait InstanceLimit {
    fn instance_limit(&mut self, limit: PValue);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + InstanceLimit,
{
    pub(in crate::svc::vast::aggr) fn instance_limit(&mut self, limit: PValue) {
        match self {
            Self::Simple(inner) => inner.instance_limit(limit),
            Self::Complex(inner) => inner.instance_limit(limit),
        }
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + InstanceLimit,
{
    fn instance_limit(&mut self, limit: PValue) {
        self.instance.instance_limit(limit);
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + InstanceLimit,
{
    fn instance_limit(&mut self, limit: PValue) {
        self.instance.instance_limit(limit);
    }
}
