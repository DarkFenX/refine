use super::{output_complex::OutputComplex, output_simple::OutputSimple, shared::OutputIterItem};
use crate::def::AttrVal;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Output<T>
where
    T: Copy,
{
    Simple(OutputSimple<T>),
    Complex(OutputComplex<T>),
}
impl<T> Output<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_amount(&self) -> T {
        match self {
            Output::Simple(inner) => inner.get_amount(),
            Output::Complex(inner) => inner.get_amount(),
        }
    }
    pub(in crate::svc) fn get_max_amount(&self) -> T {
        match self {
            Output::Simple(inner) => inner.get_max_amount(),
            Output::Complex(inner) => inner.get_max_amount(),
        }
    }
    pub(in crate::svc) fn get_completion_time(&self) -> AttrVal {
        match self {
            Output::Simple(inner) => inner.get_completion_time(),
            Output::Complex(inner) => inner.get_completion_time(),
        }
    }
    pub(in crate::svc) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        match self {
            Self::Simple(inner) => OutputIter::Simple(inner.iter_amounts()),
            Self::Complex(inner) => OutputIter::Complex(inner.iter_amounts()),
        }
    }
}
impl Output<AttrVal> {
    pub(in crate::svc) fn has_impact(&self) -> bool {
        match self {
            Output::Simple(inner) => inner.has_impact(),
            Output::Complex(inner) => inner.has_impact(),
        }
    }
    pub(in crate::svc) fn absolute_impact(&self) -> AttrVal {
        match self {
            Output::Simple(inner) => inner.absolute_impact(),
            Output::Complex(inner) => inner.absolute_impact(),
        }
    }
    pub(in crate::svc) fn add_amount(&mut self, amount: AttrVal) {
        match self {
            Output::Simple(inner) => inner.add_amount(amount),
            Output::Complex(inner) => inner.add_amount(amount),
        }
    }
}
impl<T> std::ops::Neg for Output<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Simple(inner) => Self::Simple(-inner),
            Self::Complex(inner) => Self::Complex(-inner),
        }
    }
}
impl<T> std::ops::MulAssign<AttrVal> for Output<T>
where
    T: Copy + std::ops::MulAssign<AttrVal>,
{
    fn mul_assign(&mut self, rhs: AttrVal) {
        match self {
            Self::Simple(inner) => inner.mul_assign(rhs),
            Self::Complex(inner) => inner.mul_assign(rhs),
        }
    }
}

pub(in crate::svc) enum OutputIter<S, C> {
    Simple(S),
    Complex(C),
}
impl<S, C, T> Iterator for OutputIter<S, C>
where
    S: Iterator<Item = OutputIterItem<T>>,
    C: Iterator<Item = OutputIterItem<T>>,
    T: Copy,
{
    type Item = OutputIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(inner) => inner.next(),
            Self::Complex(inner) => inner.next(),
        }
    }
}
