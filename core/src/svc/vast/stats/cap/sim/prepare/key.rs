use super::shared::SIG_ROUND_DIGITS;
use crate::{
    num::{Count, PValue},
    svc::{
        output::{Output, OutputComplex, OutputSimple},
        vast::aggr::{AggrPartDataRegular, AggrPartDataSpool},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle sequence
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct CSeqPartKey {
    duration: PValue,
    output: OutputKey,
}
impl<T: Copy> From<AggrPartDataRegular<T>> for CSeqPartKey {
    fn from(part_data: AggrPartDataRegular<T>) -> Self {
        Self {
            duration: part_data.cycle_duration.sig_rounded(SIG_ROUND_DIGITS),
            output: OutputKey::from_output(&part_data.output),
        }
    }
}
impl<T: Copy> From<AggrPartDataSpool<T>> for CSeqPartKey {
    fn from(part_data: AggrPartDataSpool<T>) -> Self {
        Self {
            duration: part_data.cycle_duration.sig_rounded(SIG_ROUND_DIGITS),
            // This one is based on base output and will yield the same output key
            output: OutputKey::from_output(&part_data.output_zero_spool),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
