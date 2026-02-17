pub(crate) use output::Output;
pub(in crate::svc) use output::OutputInstanceIter;
pub(crate) use output_complex::OutputComplex;
pub(crate) use output_simple::OutputSimple;

mod output;
mod output_complex;
mod output_simple;
mod shared;
