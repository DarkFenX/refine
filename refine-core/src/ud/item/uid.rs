use crate::util::SlabId;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UItemId(usize);

impl SlabId for UItemId {
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

    impl UItemId {
        pub(crate) fn consistency_check(&self, u_data: &UData, check_load: bool) -> DebugResult {
            let Some(item) = u_data.items.try_get(*self) else {
                return Err(Default::default());
            };
            if check_load && !item.is_loaded() {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
