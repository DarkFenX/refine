use crate::svc::output::{Output, OutputComplex, OutputSimple};

pub(crate) trait HasImpact {
    fn has_impact(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + HasImpact,
{
    pub(in crate::svc::vast::aggr) fn has_impact(&self) -> bool {
        match self {
            Self::Simple(inner) => inner.has_impact(),
            Self::Complex(inner) => inner.has_impact(),
        }
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + HasImpact,
{
    fn has_impact(&self) -> bool {
        self.instance.has_impact()
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + HasImpact,
{
    fn has_impact(&self) -> bool {
        self.instance.has_impact()
    }
}
