use crate::{sol::SolCore, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Mutation applied to an item.
pub enum Mutation<'sol> {
    Effective(EffectiveMutation<'sol>),
    Dormant(DormantMutation<'sol>),
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutation<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) item_uid: UItemId,
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct DormantMutation<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) item_uid: UItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> EffectiveMutation<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
impl<'sol> DormantMutation<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
