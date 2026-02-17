use crate::{
    num::{Count, PValue},
    svc::{
        cycle::CycleSeq,
        output::{Output, OutputComplex, OutputSimple},
        vast::aggr::{AggrIterData, AggrPartDataRegular, AggrPartDataSpool},
    },
};

pub(super) const TIME_ROUND_DIGITS: u32 = 10;

impl AggrIterData<PValue> {
    pub(super) fn extract_cseq_timing_key(&self) -> CycleSeq<CSeqPartTimingKey> {
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
pub(super) struct CSeqPartTimingKey {
    pub(super) duration: PValue,
    output: OutputTimingKey,
}
impl From<AggrPartDataRegular<PValue>> for CSeqPartTimingKey {
    fn from(part_data: AggrPartDataRegular<PValue>) -> Self {
        Self {
            duration: part_data.cycle_duration.sig_rounded(TIME_ROUND_DIGITS),
            output: OutputTimingKey::from_output(&part_data.output),
        }
    }
}
impl From<AggrPartDataSpool<PValue>> for CSeqPartTimingKey {
    fn from(part_data: AggrPartDataSpool<PValue>) -> Self {
        Self {
            duration: part_data.cycle_duration.sig_rounded(TIME_ROUND_DIGITS),
            // This one is based on base output and will yield the same output key
            output: OutputTimingKey::from_output(&part_data.output_zero_spool),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum OutputTimingKey {
    Simple(OutputTimingKeySimple),
    Complex(OutputTimingKeyComplex),
}
impl OutputTimingKey {
    fn from_output(output: &Output<PValue>) -> Self {
        match output {
            Output::Simple(inner) => OutputTimingKey::Simple(OutputTimingKeySimple::from_output(inner)),
            Output::Complex(inner) => OutputTimingKey::Complex(OutputTimingKeyComplex::from_output(inner)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct OutputTimingKeySimple {
    delay: PValue,
}
impl OutputTimingKeySimple {
    fn from_output(output: &OutputSimple<PValue>) -> Self {
        Self {
            delay: output.delay.sig_rounded(TIME_ROUND_DIGITS),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct OutputTimingKeyComplex {
    delay: PValue,
    repeats: Count,
    interval: PValue,
}
impl OutputTimingKeyComplex {
    fn from_output(output: &OutputComplex<PValue>) -> Self {
        Self {
            delay: output.delay.sig_rounded(TIME_ROUND_DIGITS),
            repeats: output.repeats,
            interval: output.interval.sig_rounded(TIME_ROUND_DIGITS),
        }
    }
}
