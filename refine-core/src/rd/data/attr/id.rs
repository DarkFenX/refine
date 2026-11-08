use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RAttrId(U32NonMax);

impl DenseId for RAttrId {
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

    impl RAttrId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            // Will crash if attr ID is not valid
            u_data.r_data.get_attr_by_rid(*self);
            Ok(())
        }
    }
}
