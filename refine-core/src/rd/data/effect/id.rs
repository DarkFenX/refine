use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct REffectId(usize);

impl SlabId for REffectId {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(self) -> usize {
        self.0
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
