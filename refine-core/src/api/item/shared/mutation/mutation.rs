use crate::{SolarSystem, ud::UItemId};

/// Mutation applied to an item.
pub enum Mutation<'s> {
    Effective(EffectiveMutation<'s>),
    Dormant(DormantMutation<'s>),
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutation<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) item_uid: UItemId,
}
impl<'s> EffectiveMutation<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct DormantMutation<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) item_uid: UItemId,
}
impl<'s> DormantMutation<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
