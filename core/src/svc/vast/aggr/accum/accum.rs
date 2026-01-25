use crate::num::{Count, PValue, Value};

pub(in crate::svc::vast) trait StatAccum<T, U> {
    fn add_amount(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
    fn get_stat(self) -> U;
}

pub(in crate::svc::vast) trait PsStatAccum<T, U> {
    fn add_amount(&mut self, output: T, chance_mult: Option<PValue>);
    fn add_amount_multiple(&mut self, output: T, chance_mult: Option<PValue>, count: Count);
    fn finalize_sequence(&mut self, duration: PValue) {}
    fn get_stat(self) -> U;
}
