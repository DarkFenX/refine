use super::{output_complex::OutputComplex, output_simple::OutputSimple};
use crate::num::{Count, PValue};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Output<I> {
    Simple(OutputSimple<I>),
    Complex(OutputComplex<I>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> Output<I> {
    pub(in crate::svc) fn get_instance(&self) -> I
    where
        I: Copy,
    {
        match self {
            Output::Simple(inner) => inner.get_instance(),
            Output::Complex(inner) => inner.get_instance(),
        }
    }
    pub(in crate::svc) fn get_instance_count(&self) -> Count {
        match self {
            Output::Simple(inner) => inner.get_instance_count(),
            Output::Complex(inner) => inner.get_instance_count(),
        }
    }
    pub(in crate::svc) fn get_immediate_instance(&self) -> Option<I>
    where
        I: Copy,
    {
        match self {
            Output::Simple(inner) => inner.get_immediate_instance(),
            Output::Complex(inner) => inner.get_immediate_instance(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> Output<I>
where
    I: std::ops::MulAssign<PValue>,
{
    pub(crate) fn instance_mul_assign(&mut self, rhs: PValue) {
        match self {
            Self::Simple(inner) => inner.instance_mul_assign(rhs),
            Self::Complex(inner) => inner.instance_mul_assign(rhs),
        }
    }
}
