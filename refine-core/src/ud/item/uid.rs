use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct UItemId(U32NonMax);
impl UItemId {
    pub(crate) const NICHE: u32 = U32NonMax::NICHE;
    pub(crate) fn into_raw(self) -> u32 {
        self.0.into_u32()
    }
}

impl DenseId for UItemId {
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
