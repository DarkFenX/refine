use std::num::NonZeroU32;

use crate::util::DenseId;

/// Imitates non-max u32, since pattern types are not really usable even in nightly.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(NonZeroU32);

impl DenseId for RAttrId {
    fn new(index: usize) -> Self {
        Self(NonZeroU32::MIN.checked_add(index.try_into().unwrap()).unwrap())
    }
    fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl RAttrId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            // Will crash if attr ID is not valid
            u_data.r_data.get_attr_by_rid(*self);
            Ok(())
        }
    }
}
