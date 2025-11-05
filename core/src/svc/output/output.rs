use super::{output_complex::OutputComplex, output_simple::OutputSimple};
use crate::def::AttrVal;

pub(crate) enum Output<T>
where
    T: Copy + Clone,
{
    Simple(OutputSimple<T>),
    Complex(OutputComplex<T>),
}
impl<T> Output<T>
where
    T: Copy + Clone,
{
    pub(in crate::svc) fn get_max(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_max(),
            Output::Complex(complex) => complex.get_max(),
        }
    }
    pub(in crate::svc) fn iter_output(&self) -> impl Iterator<Item = (AttrVal, T)> {
        match self {
            Self::Simple(simple) => OutputIter::Simple(simple.iter_output()),
            Self::Complex(complex) => OutputIter::Complex(complex.iter_output()),
        }
    }
}
impl<T> Output<T>
where
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_total(),
            Output::Complex(complex) => complex.get_total(),
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
    T: Copy + Clone,
{
    type Item = (AttrVal, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(simple) => simple.next(),
            Self::Complex(complex) => complex.next(),
        }
    }
}
