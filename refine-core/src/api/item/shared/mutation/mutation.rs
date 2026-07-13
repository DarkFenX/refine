use crate::{sol::SolarSystem, ud::UItemId};

pub enum Mutation<'s> {
    Effective(EffectiveMutation<'s>),
    Incomplete(IncompleteMutation<'s>),
}

pub enum MutationMut<'s> {
    Effective(EffectiveMutationMut<'s>),
    Incomplete(IncompleteMutationMut<'s>),
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutation<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'s> EffectiveMutation<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutationMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'s> EffectiveMutationMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct IncompleteMutation<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'s> IncompleteMutation<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct IncompleteMutationMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'s> IncompleteMutationMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
