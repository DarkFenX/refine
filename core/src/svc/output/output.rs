use super::{output_complex::OutputComplex, output_simple::OutputSimple};
use crate::{def::AttrVal, util::Limit};

#[derive(Copy, Clone)]
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
    pub(in crate::svc) fn get_max(&self) -> T {
        match self {
            Output::Simple(inner) => inner.get_max(),
            Output::Complex(inner) => inner.get_max(),
        }
    }
    pub(in crate::svc) fn get_delay(&self) -> AttrVal {
        match self {
            Output::Simple(inner) => inner.get_delay(),
            Output::Complex(inner) => inner.get_delay(),
        }
    }
    pub(in crate::svc) fn iter_output(&self) -> impl Iterator<Item = (AttrVal, T)> {
        match self {
            Self::Simple(inner) => OutputIter::Simple(inner.iter_output()),
            Self::Complex(inner) => OutputIter::Complex(inner.iter_output()),
        }
    }
}
impl<T> Output<T>
where
    T: Copy + Limit,
{
    pub(in crate::svc) fn limit_amount(&mut self, limit: AttrVal) {
        match self {
            Output::Simple(inner) => inner.limit_amount(limit),
            Output::Complex(inner) => inner.limit_amount(limit),
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
    S: Iterator<Item = (AttrVal, T)>,
    C: Iterator<Item = (AttrVal, T)>,
    T: Copy,
{
    type Item = (AttrVal, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(inner) => inner.next(),
            Self::Complex(inner) => inner.next(),
        }
    }
}
