use crate::{sol::SolarSystem, ud::UItemId};

pub enum Mutation<'a> {
    Effective(EffectiveMutation<'a>),
    Incomplete(IncompleteMutation<'a>),
}

pub enum MutationMut<'a> {
    Effective(EffectiveMutationMut<'a>),
    Incomplete(IncompleteMutationMut<'a>),
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutation<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'a> EffectiveMutation<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which has enough prerequisites met to affect item it mutates.
pub struct EffectiveMutationMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'a> EffectiveMutationMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct IncompleteMutation<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'a> IncompleteMutation<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}

/// Mutation which doesn't have enough data available, thus doesn't change anything on the item it's
/// applied to.
pub struct IncompleteMutationMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
}
impl<'a> IncompleteMutationMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, item_uid: UItemId) -> Self {
        Self { sol, item_uid }
    }
}
