use crate::{num::U32NonMax, util::DenseId};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct UFitId(U32NonMax);
impl UFitId {
    pub(crate) const NICHE: u32 = U32NonMax::NICHE;
    pub(crate) fn into_raw(self) -> u32 {
        self.0.into_u32()
    }
}

impl DenseId for UFitId {
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

    impl UFitId {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            if u_data.fits.try_get(*self).is_none() {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
