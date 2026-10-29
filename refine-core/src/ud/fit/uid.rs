use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFitId(usize);

impl SlabId for UFitId {
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

    impl UFitId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            if u_data.fits.try_get(*self).is_none() {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
