use crate::{ed::EItemId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AItemId(i32);
impl AItemId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
    pub(crate) fn new_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl const From<EItemId> for AItemId {
    fn from(item_eid: EItemId) -> Self {
        Self::new(item_eid.into_inner())
    }
}
