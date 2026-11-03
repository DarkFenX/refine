use crate::{sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Mutation applied to an item.
pub enum Mutation<'s> {
    Effective(EffectiveMutation<'s>),
    Dormant(DormantMutation<'s>),
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutation<'s> {
    pub(super) sol: &'s mut CoreInner,
    pub(super) item_uid: UItemId,
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct DormantMutation<'s> {
    pub(super) sol: &'s mut CoreInner,
    pub(super) item_uid: UItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> EffectiveMutation<'s> {
    pub(in crate::api) fn new(sol: &'s mut CoreInner, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
impl<'s> DormantMutation<'s> {
    pub(in crate::api) fn new(sol: &'s mut CoreInner, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
