use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct REffectId(U32NonMax);

// Placeholder for inline storage of small sets
impl Default for REffectId {
    fn default() -> Self {
        Self(U32NonMax::ZERO)
    }
}

impl DenseId for REffectId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl REffectId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            // Will crash if effect ID is not valid
            u_data.r_data.get_effect_by_rid(*self);
            Ok(())
        }
    }
}
