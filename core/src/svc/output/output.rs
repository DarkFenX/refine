use super::{
    output_complex::{OutputComplex, OutputInstanceIterComplex},
    output_simple::{OutputInstanceIterSimple, OutputSimple},
    shared::OutputInstanceIterItem,
};
use crate::num::{Count, PValue};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Output<T: Copy> {
    Simple(OutputSimple<T>),
    Complex(OutputComplex<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Instance iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy,
{
    pub(in crate::svc) fn into_instance_iter(self) -> OutputInstanceIter<T> {
        match self {
            Self::Simple(inner) => OutputInstanceIter::Simple(inner.into_instance_iter()),
            Self::Complex(inner) => OutputInstanceIter::Complex(inner.into_instance_iter()),
        }
    }
}

pub(in crate::svc) enum OutputInstanceIter<T>
where
    T: Copy,
{
    Simple(OutputInstanceIterSimple<T>),
    Complex(OutputInstanceIterComplex<T>),
}
impl<T> Iterator for OutputInstanceIter<T>
where
    T: Copy,
{
    type Item = OutputInstanceIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(inner) => inner.next(),
            Self::Complex(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Output<T> {
    pub(in crate::svc) fn get_instance(&self) -> T {
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
    pub(in crate::svc) fn get_immediate_instance(&self) -> Option<T> {
        match self {
            Output::Simple(inner) => inner.get_immediate_instance(),
            Output::Complex(inner) => inner.get_immediate_instance(),
        }
    }
    pub(in crate::svc) fn limit_duration(&self, duration: PValue) -> Option<Output<T>> {
        match self {
            Output::Simple(inner) => inner.limit_duration(duration),
            Output::Complex(inner) => inner.limit_duration(duration),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    pub(crate) fn instance_mul_assign(&mut self, rhs: PValue) {
        match self {
            Self::Simple(inner) => inner.instance_mul_assign(rhs),
            Self::Complex(inner) => inner.instance_mul_assign(rhs),
        }
    }
}
