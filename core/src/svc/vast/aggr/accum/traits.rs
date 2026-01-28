use crate::num::{Count, PValue};

// All effects' stats are considered over the same period of time (time aggregators)
pub(in crate::svc::vast) trait StatAccumSynced<T> {
    fn add_amount(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
}

// All effects' stats are considered over different periods of time (first cycle & clip aggregators)
pub(in crate::svc::vast) trait StatAccumUnsynced<T> {
    fn add_amount(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
    fn finalize_sequence(&mut self, duration: PValue) {}
}

// All effects' stats are considered over looped part of sequence, which can have different
// durations for every effect (loop aggregators)
pub(in crate::svc::vast) trait StatAccumLooped<T> {
    fn add_amount_preloop(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_preloop_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
    fn add_amount_loop(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_loop_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
    fn finalize_loop_sequence(&mut self, duration: PValue) {}
}
