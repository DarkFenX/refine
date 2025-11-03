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
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_total(),
            Output::Complex(complex) => complex.get_total(),
        }
    }
    pub(in crate::svc) fn get_max(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_max(),
            Output::Complex(complex) => complex.get_max(),
        }
    }
}
