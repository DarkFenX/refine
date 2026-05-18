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
impl<I> Output<I> {
    pub(in crate::svc::vast::aggr) fn instance_limit(&mut self, limit: PValue)
    where
        I: InstanceLimit,
    {
        match self {
            Self::Simple(inner) => inner.instance_limit(limit),
            Self::Complex(inner) => inner.instance_limit(limit),
        }
    }
}
impl<I> OutputSimple<I> {
    fn instance_limit(&mut self, limit: PValue)
    where
        I: InstanceLimit,
    {
        self.instance.instance_limit(limit);
    }
}
impl<I> OutputComplex<I> {
    fn instance_limit(&mut self, limit: PValue)
    where
        I: InstanceLimit,
    {
        self.instance.instance_limit(limit);
    }
}
