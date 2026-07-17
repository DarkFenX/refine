use crate::num::PValue;

/// Range for jump stats used for fuel use calculations.
#[derive(Copy, Clone, Default)]
pub enum StatJumpRange {
    LightYears(PValue),
    #[default]
    Max,
}
