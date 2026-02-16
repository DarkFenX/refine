use crate::{
    num::{Count, PValue},
    svc::{
        cycle::CycleSeq,
        output::{Output, OutputComplex, OutputSimple},
        vast::aggr::{AggrIterData, AggrPartDataRegular, AggrPartDataSpool},
    },
};

const SIG_ROUND_DIGITS: u32 = 10;

impl AggrIterData<PValue> {
    pub(super) fn extract_cseq(&self) -> CycleSeq<CSeqPartKey> {
        match self {
            Self::Regular(inner) => inner.cseq.convert_and_optimize(),
            Self::Spool(inner) => inner.cseq.convert_and_optimize(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle sequence
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct CSeqPartKey {
    pub(super) duration: PValue,
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
enum OutputKey {
    Simple(OutputKeySimple),
    Complex(OutputKeyComplex),
}
impl OutputKey {
    fn from_output<T: Copy>(output: &Output<T>) -> Self {
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
