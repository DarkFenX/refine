use super::shared::SIG_ROUND_DIGITS;
use crate::{
    num::{Count, PValue},
    svc::output::{Output, OutputComplex, OutputSimple},
};

#[derive(Eq, PartialEq, Hash)]
pub(super) enum OutputKey {
    Simple(OutputKeySimple),
    Complex(OutputKeyComplex),
}
impl OutputKey {
    pub(super) fn from_output<T: Copy>(output: &Output<T>) -> Self {
        match output {
            Output::Simple(inner) => OutputKey::Simple(OutputKeySimple::from_output(inner)),
            Output::Complex(inner) => OutputKey::Complex(OutputKeyComplex::from_output(inner)),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct OutputKeySimple {
    delay: PValue,
}
impl OutputKeySimple {
    fn from_output<T: Copy>(output: &OutputSimple<T>) -> Self {
        Self {
            delay: output.delay.sig_rounded(SIG_ROUND_DIGITS),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct OutputKeyComplex {
    delay: PValue,
    repeats: Count,
    interval: PValue,
}
impl OutputKeyComplex {
    fn from_output<T: Copy>(output: &OutputComplex<T>) -> Self {
        Self {
            delay: output.delay.sig_rounded(SIG_ROUND_DIGITS),
            repeats: output.repeats,
            interval: output.interval.sig_rounded(SIG_ROUND_DIGITS),
        }
    }
}
