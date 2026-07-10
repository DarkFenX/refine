use crate::svc::output::{Output, OutputComplex, OutputSimple};

pub(crate) trait HasImpact {
    fn has_impact(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> Output<I> {
    pub(in crate::svc::vast::aggr) fn has_impact(&self) -> bool
    where
        I: HasImpact,
    {
        match self {
            Self::Simple(inner) => inner.has_impact(),
            Self::Complex(inner) => inner.has_impact(),
        }
    }
}
impl<I> OutputSimple<I> {
    fn has_impact(&self) -> bool
    where
        I: HasImpact,
    {
        self.instance.has_impact()
    }
}
impl<I> OutputComplex<I> {
    fn has_impact(&self) -> bool
    where
        I: HasImpact,
    {
        self.instance.has_impact()
    }
}
