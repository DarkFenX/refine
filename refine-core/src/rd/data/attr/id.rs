use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(usize);

impl SlabId for RAttrId {
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

    impl RAttrId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            // Will crash if attr ID is not valid
            u_data.r_data.get_attr_by_rid(*self);
            Ok(())
        }
    }
}
