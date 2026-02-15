use crate::{
    num::Value,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(crate) trait LimitInstance {
    fn limit_instance(&mut self, limit: Value);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + LimitInstance,
{
    pub(in crate::svc::vast::aggr) fn limit_instance(&mut self, limit: Value) {
        match self {
            Self::Simple(inner) => inner.limit_instance(limit),
            Self::Complex(inner) => inner.limit_instance(limit),
        }
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + LimitInstance,
{
    fn limit_instance(&mut self, limit: Value) {
        self.instance.limit_instance(limit);
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + LimitInstance,
{
    fn limit_instance(&mut self, limit: Value) {
        self.instance.limit_instance(limit);
    }
}
