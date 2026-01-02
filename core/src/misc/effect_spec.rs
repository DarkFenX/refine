use crate::{rd::REffectId, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UItemId,
    pub(crate) effect_key: REffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UItemId, effect_key: REffectId) -> Self {
        Self { item_key, effect_key }
    }
}
