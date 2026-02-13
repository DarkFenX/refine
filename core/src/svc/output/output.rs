use super::{
    output_complex::{OutputComplex, OutputInstanceIterComplex},
    output_simple::{OutputInstanceIterSimple, OutputSimple},
    shared::OutputInstanceIterItem,
};
use crate::num::{Count, PValue, Value};

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
    pub(in crate::svc) fn iter_instances(&self) -> OutputInstanceIter<'_, T> {
        match self {
            Self::Simple(inner) => OutputInstanceIter::Simple(inner.iter_instances()),
            Self::Complex(inner) => OutputInstanceIter::Complex(inner.iter_instances()),
        }
    }
}

pub(in crate::svc) enum OutputInstanceIter<'a, T>
where
    T: Copy,
{
    Simple(OutputInstanceIterSimple<'a, T>),
    Complex(OutputInstanceIterComplex<'a, T>),
}
impl<'a, T> Iterator for OutputInstanceIter<'a, T>
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
}
impl Output<Value> {
    pub(in crate::svc) fn get_absolute_impact(&self) -> PValue {
        match self {
            Output::Simple(inner) => inner.get_absolute_impact(),
            Output::Complex(inner) => inner.get_absolute_impact(),
        }
    }
    pub(in crate::svc) fn add_instance(&mut self, instance: Value) {
        match self {
            Output::Simple(inner) => inner.add_instance(instance),
            Output::Complex(inner) => inner.add_instance(instance),
        }
    }
}
impl Output<PValue> {
    pub(in crate::svc) fn has_impact(&self) -> bool {
        match self {
            Output::Simple(inner) => inner.has_impact(),
            Output::Complex(inner) => inner.has_impact(),
        }
    }
}
impl<T> std::ops::Mul<PValue> for Output<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        match self {
            Self::Simple(inner) => Self::Simple(inner * rhs),
            Self::Complex(inner) => Self::Complex(inner * rhs),
        }
    }
}
impl<T> std::ops::MulAssign<PValue> for Output<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    fn mul_assign(&mut self, rhs: PValue) {
        match self {
            Self::Simple(inner) => inner.mul_assign(rhs),
            Self::Complex(inner) => inner.mul_assign(rhs),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions of inner type
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Output<PValue> {
    pub(in crate::svc) fn into_value(self) -> Output<Value> {
        match self {
            Self::Simple(inner) => Output::Simple(inner.into_value()),
            Self::Complex(inner) => Output::Complex(inner.into_value()),
        }
    }
}
impl<T, U> std::ops::Neg for Output<T>
where
    T: Copy + std::ops::Neg<Output = U>,
    U: Copy,
{
    type Output = Output<U>;

    fn neg(self) -> Output<U> {
        match self {
            Self::Simple(inner) => Output::Simple(-inner),
            Self::Complex(inner) => Output::Complex(-inner),
        }
    }
}
