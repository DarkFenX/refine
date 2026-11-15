use crate::{ud::UItemId, util::RSet};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Status of an item which fails a validation.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValItemStatus {
    Failed,
    /// Item fails the validation, but was passed as known failure.
    Kf,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValItemStatus {
    pub(in crate::svc::vast::val) fn from_kfs(kfs: &RSet<UItemId>, item_uid: UItemId) -> Self {
        match kfs.contains(&item_uid) {
            true => Self::Kf,
            false => Self::Failed,
        }
    }
    pub(in crate::svc::vast::val) fn is_kf(self) -> bool {
        matches!(self, Self::Kf)
    }
}
