use crate::misc::PValue;

#[derive(Copy, Clone)]
pub struct MiningAmount {
    pub yield_: PValue,
    pub drain: PValue,
}
impl Default for MiningAmount {
    fn default() -> Self {
        Self {
            yield_: PValue::ZERO,
            drain: PValue::ZERO,
        }
    }
}
