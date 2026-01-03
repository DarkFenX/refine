use crate::{rd::REffectId, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_uid: UItemId,
    pub(crate) effect_rid: REffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_uid: UItemId, effect_rid: REffectId) -> Self {
        Self { item_uid, effect_rid }
    }
}
